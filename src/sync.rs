use crate::config;
use crate::error::ZeusError;
use crate::log::{self, Level};

use bollard::Docker;

use bollard::container::{
    AttachContainerOptions, Config, CreateContainerOptions, ListContainersOptions,
    StartContainerOptions,
};

use bollard::models::{
    HostConfig, Mount, MountBindOptions, MountBindOptionsPropagationEnum, MountTypeEnum,
};

use futures::StreamExt;

use std::fs::{self, remove_file};
use std::os::unix::fs::PermissionsExt;

use std::io::prelude::*;
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};

struct Listener {
    listener: UnixListener,
    path: PathBuf,
}

impl Drop for Listener {
    // unlink unix socket
    fn drop(&mut self) {
        let _ = remove_file(self.path.as_path());
    }
}

pub async fn sync(
    logger: &mut log::Logger,
    docker: Docker,
    cfg: config::Config,
) -> Result<(), ZeusError> {
    let socket_path = format!("{}/zeus.sock", &cfg.build_dir);

    if !Path::new(&cfg.build_dir).exists() {
        return Err(ZeusError::new(
            "filesystem",
            format!("Package build directory does not exist: {}", &cfg.build_dir),
        ));
    }

    logger.v(
        Level::Verbose,
        "unix",
        format!("Opening socket for builder: {}", socket_path),
    );

    let _ = remove_file(&socket_path);
    let listener = Listener {
        path: Path::new(&socket_path).to_owned(),
        listener: match UnixListener::bind(&socket_path) {
            Ok(v) => {
                let _ = fs::set_permissions(&socket_path, fs::Permissions::from_mode(0o666));
                v
            }
            Err(e) => {
                return Err(ZeusError::new(
                    "unix",
                    format!("Cannot listen on socket: {}", e),
                ));
            }
        },
    };

    let opts = ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    };

    let mut should_create = true;

    logger.v(Level::Verbose, "docker", "Querying containers...");

    match docker.list_containers(Some(opts)).await {
        Err(e) => {
            return Err(ZeusError::new(
                "docker",
                format!("Cannot query containers: {}", e),
            ));
        }
        Ok(v) => {
            for container in v {
                if let Some(names) = &container.names {
                    if names.contains(&format!("/{}", &cfg.builder.name)) {
                        should_create = false;
                        break;
                    }
                }
            }
        }
    }

    if should_create {
        let opts = CreateContainerOptions {
            name: &cfg.builder.name,
        };

        let config = Config {
            image: Some(cfg.builder.image.clone()),

            tty: Some(true),

            host_config: Some(HostConfig {
                privileged: Some(false),
                cap_drop: Some(vec!["all".to_owned()]),
                cap_add: Some(vec!["CAP_SETUID".to_owned(), "CAP_SETGID".to_owned()]), // needed for sudo
                //security_opt: Some(vec!["no-new-privileges:true".to_owned()]), // conflicts with sudo
                mounts: Some(vec![Mount {
                    typ: Some(MountTypeEnum::BIND),
                    source: Some(cfg.build_dir.clone()),
                    target: Some("/build".to_owned()),
                    read_only: Some(false),
                    bind_options: Some(MountBindOptions {
                        propagation: Some(MountBindOptionsPropagationEnum::RPRIVATE),
                        ..Default::default()
                    }),
                    ..Default::default()
                }]),
                ..Default::default()
            }),
            ..Default::default()
        };

        logger.v(Level::Verbose, "docker", "Creating builder...");

        match docker.create_container(Some(opts), config).await {
            Ok(_) => {}
            Err(e) => {
                return Err(ZeusError::new(
                    "docker",
                    format!("Error creating builder: {}", e),
                ));
            }
        }
    } else {
        logger.v(
            Level::Verbose,
            "docker",
            "Builder already exists! Not creating a new one...",
        );
    }

    logger.v(Level::Verbose, "zeus", "Starting builder...");

    let opts = StartContainerOptions::<String> {
        ..Default::default()
    };

    match docker.start_container(&cfg.builder.name, Some(opts)).await {
        Ok(_) => {}
        Err(e) => {
            return Err(ZeusError::new(
                "docker",
                format!("Error starting builder: {}", e),
            ));
        }
    }

    logger.v(Level::Verbose, "zeus", "Waiting for builder...");

    let mut stream = match listener.listener.accept() {
        Ok(v) => v.0,
        Err(e) => {
            return Err(ZeusError::new(
                "unix",
                format!("Cannot open communication stream with builder: {}", e),
            ));
        }
    };

    let data = match serde_json::to_string(&cfg) {
        Ok(v) => v,
        Err(e) => {
            return Err(ZeusError::new(
                "zeus",
                format!("Cannot serialize builder data: {}", e),
            ));
        }
    };

    match write!(&mut stream, "{}", data) {
        Ok(_) => {}
        Err(e) => {
            return Err(ZeusError::new(
                "zeus",
                format!("Cannot send package information to builder: {}", e),
            ));
        }
    }

    logger.v(Level::Verbose, "zeus", "Attaching to builder...");

    let opts = AttachContainerOptions::<String> {
        stdin: Some(true),
        stdout: Some(true),
        stderr: Some(true),
        stream: Some(true),
        logs: Some(true),
        ..Default::default()
    };

    match docker.attach_container(&cfg.builder.name, Some(opts)).await {
        Ok(mut v) => {
            while let Some(res) = v.output.next().await {
                match res {
                    Ok(v) => print!("{}", v),
                    Err(e) => {
                        return Err(ZeusError::new(
                            "docker",
                            format!("Error displaying builder logs: {}", e),
                        ));
                    }
                }
            }
        }
        Err(e) => {
            return Err(ZeusError::new(
                "zeus",
                format!("Cannot attach to builder: {}", e),
            ));
        }
    }

    Ok(())
}

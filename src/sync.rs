use crate::config;
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

use std::io::prelude::*;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::process::exit;

pub async fn sync(mut logger: log::Logger, docker: Docker, cfg: config::Config) {
    let container_name = "zeus-builder".to_owned();
    let socket_path = format!("{}/zeus.sock", &cfg.build_dir);

    if !Path::new(&cfg.build_dir).exists() {
        logger.v(
            Level::Error,
            "zeus",
            format!("Package build directory does not exist: {}", &cfg.build_dir),
        );
        return;
    }

    if cfg.verbose {
        logger.v(
            Level::Verbose,
            "zeus",
            format!("Opening socket for builder: {}", socket_path),
        );
    }

    let _ = std::fs::remove_file(&socket_path);
    let listener = match UnixListener::bind(&socket_path) {
        Ok(v) => v,
        Err(e) => {
            logger.v(
                Level::Error,
                "zeus",
                format!("Cannot listen on socket: {}", e),
            );
            return;
        }
    };

    let opts = ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    };

    let mut should_create = true;
    if cfg.verbose {
        logger.v(Level::Verbose, "docker", "Querying containers...");
    }

    match docker.list_containers(Some(opts)).await {
        Err(e) => {
            logger.v(
                Level::Error,
                "docker",
                format!("Cannot list containers: {}", e),
            );
            return;
        }
        Ok(v) => {
            for container in v {
                if let Some(names) = &container.names {
                    if names.contains(&format!("/{}", &container_name)) {
                        should_create = false;
                        break;
                    }
                }
            }
        }
    }

    if should_create {
        let opts = CreateContainerOptions {
            name: &container_name,
        };

        let config = Config {
            image: Some(cfg.builder_image.clone()),

            tty: Some(true),

            host_config: Some(HostConfig {
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

        if cfg.verbose {
            logger.v(Level::Verbose, "zeus", "Creating builder...");
        }

        match docker.create_container(Some(opts), config).await {
            Ok(_) => {}
            Err(e) => {
                if cfg.verbose {
                    logger.v(
                        Level::Verbose,
                        "docker",
                        format!("Error creating builder: {}", e),
                    );
                }
            }
        }
    } else {
        if cfg.verbose {
            logger.v(
                Level::Verbose,
                "docker",
                "Builder already exists! Not creating a new one...",
            );
        }
    }

    if cfg.verbose {
        logger.v(Level::Verbose, "zeus", "Starting builder...");
    }

    let opts = StartContainerOptions::<String> {
        ..Default::default()
    };

    match docker.start_container(&container_name, Some(opts)).await {
        Ok(_) => {}
        Err(e) => {
            logger.v(
                Level::Error,
                "docker",
                format!("Error starting builder: {}", e),
            );
            return;
        }
    }

    if cfg.verbose {
        logger.v(Level::Verbose, "zeus", "Waiting for builder...");
    }

    let mut stream = match listener.accept() {
        Ok(v) => v.0,
        Err(e) => {
            logger.v(
                Level::Error,
                "zeus",
                format!("Cannot open communication stream with builder: {}", e),
            );
            return;
        }
    };

    let data = match serde_json::to_string(&cfg) {
        Ok(v) => v,
        Err(e) => {
            logger.v(
                Level::Error,
                "zeus",
                format!("Cannot serialize builder data: {}", e),
            );
            return;
        }
    };

    match write!(&mut stream, "{}", data) {
        Ok(_) => {}
        Err(e) => {
            logger.v(
                Level::Error,
                "zeus",
                format!("Cannot send package information to builder: {}", e),
            );
            return;
        }
    }

    if cfg.verbose {
        logger.v(Level::Verbose, "zeus", "Attaching to builder...");
    }

    let opts = AttachContainerOptions::<String> {
        stdin: Some(true),
        stdout: Some(true),
        stderr: Some(true),
        stream: Some(true),
        ..Default::default()
    };

    match docker.attach_container(&container_name, Some(opts)).await {
        Ok(mut v) => {
            while let Some(res) = v.output.next().await {
                match res {
                    Ok(v) => print!("{}", v),
                    Err(e) => {
                        logger.v(
                            Level::Error,
                            "docker",
                            format!("Error displaying builder logs: {}", e),
                        );
                        exit(1);
                    }
                }
            }
        }
        Err(e) => {
            logger.v(
                Level::Error,
                "zeus",
                format!("Cannot attach to builder: {}", e),
            );
            exit(1);
        }
    }
}

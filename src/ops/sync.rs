use crate::config;
use crate::error::{zerr, ZeusError};
use crate::log::{self, Level};
use crate::util::LocalListener;

use bollard::Docker;

use bollard::container::{
	AttachContainerOptions, Config, CreateContainerOptions, KillContainerOptions,
	ListContainersOptions, StartContainerOptions,
};
use bollard::models::{
	HostConfig, Mount, MountBindOptions, MountBindOptionsPropagationEnum, MountTypeEnum,
};

use futures::StreamExt;

use ctrlc;

use std::io::prelude::*;
use std::path::Path;
use std::sync::mpsc::channel;

pub async fn sync(
	logger: &mut log::Logger,
	docker: Docker,
	cfg: config::AppConfig,
) -> Result<(), ZeusError> {
	let socket_path = format!("{}/zeus.sock", &cfg.builddir);

	logger.v(
		Level::Verbose,
		format!("Opening socket for builder: {}", socket_path),
	);

	let listener = zerr!(
		LocalListener::new(Path::new(&socket_path), Some(0o666)),
		"Cannot listen on socket: "
	);

	let opts = ListContainersOptions::<String> {
		all: true,
		..Default::default()
	};

	let mut should_create = true;

	logger.v(Level::Verbose, "Querying created containers...");

	let container_list = zerr!(
		docker.list_containers(Some(opts)).await,
		"Cannot query containers: "
	);

	for container in container_list {
		if let Some(names) = &container.names {
			if names.contains(&format!("/{}", &cfg.name)) {
				should_create = false;
				break;
			}
		}
	}

	#[cfg(debug_assertions)]
	logger.v(Level::Debug, format!("should_create = {:?}", should_create));

	if should_create {
		let opts = CreateContainerOptions { name: &cfg.name };

		let config = Config {
			image: Some(cfg.image.clone()),

			tty: Some(true),

			host_config: Some(HostConfig {
				privileged: Some(false),
				cap_drop: Some(vec!["all".to_owned()]),
				cap_add: Some(vec!["CAP_SETUID".to_owned(), "CAP_SETGID".to_owned()]), // needed for sudo
				//security_opt: Some(vec!["no-new-privileges:true".to_owned()]), // conflicts with sudo
				mounts: Some(vec![
					Mount {
						typ: Some(MountTypeEnum::BIND),
						source: Some("/var/cache/pacman/pkg".to_owned()),
						target: Some("/var/cache/pacman/pkg".to_owned()),
						read_only: Some(false),
						bind_options: Some(MountBindOptions {
							propagation: Some(MountBindOptionsPropagationEnum::RPRIVATE),
							..Default::default()
						}),
						..Default::default()
					},
					Mount {
						typ: Some(MountTypeEnum::BIND),
						source: Some(cfg.builddir.clone()),
						target: Some("/build".to_owned()),
						read_only: Some(false),
						bind_options: Some(MountBindOptions {
							propagation: Some(MountBindOptionsPropagationEnum::RPRIVATE),
							..Default::default()
						}),
						..Default::default()
					},
				]),
				..Default::default()
			}),
			..Default::default()
		};

		logger.v(Level::Verbose, "Creating builder...");

		zerr!(
			docker.create_container(Some(opts), config).await,
			"Error creating builder: "
		);
	} else {
		logger.v(
			Level::Verbose,
			"Builder already exists! Not creating a new one...",
		);
	}

	logger.v(Level::Verbose, "Starting builder...");

	let opts = StartContainerOptions::<String> {
		..Default::default()
	};

	zerr!(
		docker.start_container(&cfg.name, Some(opts)).await,
		"Error starting builder: "
	);

	logger.v(Level::Verbose, "Waiting for builder...");

	let mut stream = zerr!(
		listener.listener.accept(),
		"Cannot open communication stream with builder: "
	)
	.0;

	let data = zerr!(
		serde_json::to_string(&cfg),
		"Cannot serialize builder data: "
	);

	zerr!(
		write!(&mut stream, "{}", data),
		"Cannot send package information to builder: "
	);

	logger.v(Level::Verbose, "Attaching to builder...");

	let opts = AttachContainerOptions::<String> {
		stdin: Some(true),
		stdout: Some(true),
		stderr: Some(true),
		stream: Some(true),
		//logs: Some(true), // this displays all output logs from container creation, thats bad
		..Default::default()
	};

	let (tx, rx) = channel();
	zerr!(
		ctrlc::set_handler(move || tx.send(()).expect("Cannot send signal")),
		"Cannot set signal handler: "
	);

	let mut out_stream = zerr!(
		docker.attach_container(&cfg.name, Some(opts)).await,
		"Cannot attach to builder: "
	)
	.output;
	while let Some(res) = out_stream.next().await {
		// This means the signal handler above triggered
		if rx.try_recv().is_ok() {
			logger.v(Level::Info, "Interrupt detected. Exiting...");

			zerr!(
				docker
					.kill_container(&cfg.name, Some(KillContainerOptions { signal: "SIGKILL" }))
					.await,
				"Cannot kill builder: "
			);
		}

		print!("{}", zerr!(res, "Error displaying builder logs: "));
	}

	Ok(())
}

use crate::config;
use crate::error::ZeusError;
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
		"unix",
		format!("Opening socket for builder: {}", socket_path),
	);

	let listener = match LocalListener::new(Path::new(&socket_path), Some(0o666)) {
		Ok(v) => v,
		Err(e) => {
			return Err(ZeusError::new(
				"unix",
				format!("Cannot listen on socket: {}", e),
			));
		}
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
					if names.contains(&format!("/{}", &cfg.name)) {
						should_create = false;
						break;
					}
				}
			}
		}
	}

	#[cfg(debug_assertions)]
	logger.v(
		Level::Debug,
		config::PROGRAM_NAME,
		format!("should_create = {:?}", should_create),
	);

	if should_create {
		let opts = CreateContainerOptions {
			name: &cfg.name,
		};

		let config = Config {
			image: Some(cfg.image.clone()),

			tty: Some(true),

			host_config: Some(HostConfig {
				privileged: Some(false),
				cap_drop: Some(vec!["all".to_owned()]),
				cap_add: Some(vec!["CAP_SETUID".to_owned(), "CAP_SETGID".to_owned()]), // needed for sudo
				//security_opt: Some(vec!["no-new-privileges:true".to_owned()]), // conflicts with sudo
				mounts: Some(vec![Mount {
					typ: Some(MountTypeEnum::BIND),
					source: Some(cfg.builddir.clone()),
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

	logger.v(Level::Verbose, config::PROGRAM_NAME, "Starting builder...");

	let opts = StartContainerOptions::<String> {
		..Default::default()
	};

	match docker.start_container(&cfg.name, Some(opts)).await {
		Ok(_) => {}
		Err(e) => {
			return Err(ZeusError::new(
				"docker",
				format!("Error starting builder: {}", e),
			));
		}
	}

	logger.v(
		Level::Verbose,
		config::PROGRAM_NAME,
		"Waiting for builder...",
	);

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
				config::PROGRAM_NAME,
				format!("Cannot serialize builder data: {}", e),
			));
		}
	};

	match write!(&mut stream, "{}", data) {
		Ok(_) => {}
		Err(e) => {
			return Err(ZeusError::new(
				config::PROGRAM_NAME,
				format!("Cannot send package information to builder: {}", e),
			));
		}
	}

	logger.v(
		Level::Verbose,
		config::PROGRAM_NAME,
		"Attaching to builder...",
	);

	let opts = AttachContainerOptions::<String> {
		stdin: Some(true),
		stdout: Some(true),
		stderr: Some(true),
		stream: Some(true),
		//logs: Some(true), // this displays all output logs from container creation, thats bad
		..Default::default()
	};

	let (tx, rx) = channel();
	match ctrlc::set_handler(move || tx.send(()).expect("Cannot send signal")) {
		Ok(_) => {}
		Err(e) => {
			return Err(ZeusError::new(
				"system",
				format!("Cannot set signal handler: {}", e),
			));
		}
	}

	match docker.attach_container(&cfg.name, Some(opts)).await {
		Ok(mut v) => {
			while let Some(res) = v.output.next().await {
				// This means the signal handler above triggered
				if rx.try_recv().is_ok() {
					logger.v(
						Level::Info,
						config::PROGRAM_NAME,
						"Interrupt detected. Exiting...",
					);

					match docker
						.kill_container(
							&cfg.name,
							Some(KillContainerOptions { signal: "SIGKILL" }),
						)
						.await
					{
						Ok(_) => {
							logger.v(Level::Success, "docker", "Killed builder");
							break;
						}
						Err(e) => {
							return Err(ZeusError::new(
								"docker",
								format!("Cannot kill builder: {}", e),
							));
						}
					}
				}

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
				config::PROGRAM_NAME,
				format!("Cannot attach to builder: {}", e),
			));
		}
	}

	Ok(())
}

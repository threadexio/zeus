use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::sync::mpsc::channel;

use bollard::container::{
	AttachContainerOptions, AttachContainerResults, Config,
	CreateContainerOptions, KillContainerOptions,
	ListContainersOptions, StartContainerOptions,
};
use bollard::models::{
	HostConfig, Mount, MountBindOptions,
	MountBindOptionsPropagationEnum, MountTypeEnum,
};

use futures::StreamExt;
use std::collections::HashMap;

use ctrlc;

use crate::util::LocalListener;

use crate::ops::prelude::*;

pub async fn sync(
	logger: &Logger,
	docker: Docker,
	cfg: &mut config::AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	cfg.upgrade = args.is_present("upgrade");

	cfg.buildargs = args
		.value_of("buildargs")
		.unwrap_or_default()
		.split_ascii_whitespace()
		.map(|x| x.to_owned())
		.collect();

	cfg.image = args.value_of("image").unwrap().to_owned();
	cfg.name = args.value_of("name").unwrap().to_owned();

	cfg.packages = args
		.values_of("packages")
		.unwrap_or_default()
		.map(|x| x.to_owned())
		.collect();

	if cfg.packages.is_empty() && cfg.upgrade {
		for pkg in args.values_of("packages").unwrap_or_default() {
			cfg.packages.insert(pkg.to_owned());
		}

		let dir = zerr!(
			fs::read_dir(&cfg.builddir),
			"fs",
			&format!("Cannot list {}", &cfg.builddir)
		);

		let mut available_packages: HashMap<usize, String> =
			HashMap::new();

		// TODO: Beautify the prompt
		println!("Choose which packages to upgrade: ");
		for (i, p) in dir
			.filter_map(|x| x.ok())
			.filter(|x| x.path().is_dir())
			.map(|x| x.file_name())
			.enumerate()
		{
			if let Some(pkg) = p.to_str() {
				available_packages.insert(i, pkg.to_owned());
				println!("{} {}", i, pkg);
			}
		}

		let mut choices = String::new();
		zerr!(
			io::stdin().read_line(&mut choices),
			"console",
			"Cannot read input"
		);

		if !choices.trim().is_empty() {
			for choice in choices.split_ascii_whitespace() {
				let choice_num: usize = match choice.parse() {
					Ok(v) => v,
					Err(_) => continue,
				};

				if available_packages.contains_key(&choice_num) {
					cfg.packages.insert(
						available_packages
							.get(&choice_num)
							.unwrap()
							.to_owned(),
					);
				}
			}
		} else {
			for (_, pkg) in available_packages {
				cfg.packages.insert(pkg);
			}
		}

		if cfg.packages.is_empty() {
			return Err(ZeusError::new(
				"zeus".to_owned(),
				"No packages specified. Exiting...".to_owned(),
			));
		}
	} else if cfg.packages.is_empty() {
		return Err(ZeusError::new(
			"zeus".to_owned(),
			"No packages specified.".to_owned(),
		));
	}

	log_debug!(logger, "debug", "{:?}", &cfg);

	let socket_path = format!("{}/zeus.sock", &cfg.builddir);

	let listener = zerr!(
		LocalListener::new(Path::new(&socket_path), Some(0o666)),
		"unix",
		format!("Cannot listen on socket {}", &socket_path)
	);

	let opts = ListContainersOptions::<String> {
		all: true,
		..Default::default()
	};

	let mut should_create = true;

	let container_list = zerr!(
		docker.list_containers(Some(opts)).await,
		"docker",
		"Cannot query containers"
	);

	for container in container_list {
		if let Some(names) = &container.names {
			if names.contains(&format!("/{}", &cfg.name)) {
				should_create = false;
				break;
			}
		}
	}

	log_info!(logger, "debug", "should_create = {:?}", should_create);

	if should_create {
		let opts = CreateContainerOptions { name: &cfg.name };

		let config = Config {
			image: Some(cfg.image.clone()),

			tty: Some(true),

			host_config: Some(HostConfig {
				privileged: Some(false),
				cap_drop: Some(vec!["all".to_owned()]),
				cap_add: Some(vec![
					"CAP_SETUID".to_owned(),
					"CAP_SETGID".to_owned(),
				]), // needed for sudo
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

		zerr!(
			docker.create_container(Some(opts), config).await,
			"docker",
			"Error creating builder"
		);
	} else {
		log_info!(logger, "docker", "Builder already exists!");
	}

	let opts =
		StartContainerOptions::<String> { ..Default::default() };

	zerr!(
		docker.start_container(&cfg.name, Some(opts)).await,
		"docker",
		"Error starting builder"
	);

	log_info!(
		logger,
		"docker",
		"Waiting for builder to come online..."
	);

	let mut stream = zerr!(
		listener.listener.accept(),
		"unix",
		"Cannot open communication stream with builder"
	)
	.0;

	match stream.set_nonblocking(true) {
		Ok(_) => {},
		Err(e) => logger
			.w("unix", format!("Cannot use non-blocking IO: {}", e)),
	};

	log_info!(logger, "docker", "Attaching to builder...");

	let data = zerr!(
		serde_json::to_string(&cfg),
		"unix",
		"Cannot send data to builder"
	);

	zerr!(
		stream.write_all(&mut data.as_bytes()),
		"unix",
		"Cannot send data to builder"
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
	zerr!(
		ctrlc::set_handler(move || tx
			.send(())
			.expect("Cannot send signal")),
		"system",
		"Cannot set signal handler"
	);

	let AttachContainerResults { output: mut out_stream, .. } = zerr!(
		docker.attach_container(&cfg.name, Some(opts)).await,
		"docker",
		"Cannot attach to builder"
	);

	while let Some(res) = out_stream.next().await {
		// This means the signal handler above triggered
		if rx.try_recv().is_ok() {
			log_info!(
				logger,
				"system",
				"Interrupt detected. Exiting..."
			);

			zerr!(
				docker
					.kill_container(
						&cfg.name,
						Some(KillContainerOptions {
							signal: "SIGKILL"
						})
					)
					.await,
				"docker",
				"Cannot kill builder"
			);
		}

		print!(
			"{}",
			zerr!(res, "docker", "Error displaying builder logs")
		);
	}

	Ok(())
}

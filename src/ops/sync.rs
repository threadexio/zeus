use crate::config;
use crate::error::{zerr, Result, ZeusError};
use crate::log::{self, Level};
use crate::util::LocalListener;

use bollard::container::{
	AttachContainerOptions, Config, CreateContainerOptions,
	KillContainerOptions, ListContainersOptions,
	StartContainerOptions,
};
use bollard::models::{
	HostConfig, Mount, MountBindOptions,
	MountBindOptionsPropagationEnum, MountTypeEnum,
};

use clap::ArgMatches;
use futures::StreamExt;
use std::collections::HashMap;
use std::process::exit;

use ctrlc;

use std::fs;
use std::io::prelude::*;
use std::path;
use std::sync::mpsc::channel;

pub async fn sync(
	logger: &mut log::Logger,
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

	if cfg.packages.is_empty() && cfg.upgrade {
		for pkg in args.values_of("packages").unwrap_or_default() {
			cfg.packages.insert(pkg.to_owned());
		}

		let dir = zerr!(
			fs::read_dir(&cfg.builddir),
			"Cannot list build directory: "
		);

		let mut available_packages: HashMap<usize, String> =
			HashMap::new();
		for (i, p) in dir
			.filter_map(|x| x.ok())
			.filter(|x| x.path().is_dir())
			.map(|x| x.file_name())
			.enumerate()
		{
			if let Some(v) = p.to_str() {
				available_packages.insert(i, v.to_owned());
			}
		}

		// TODO: Maybe sort these numerically?
		logger.v(
			Level::Info,
			format!(
				"Choose which packages to upgrade:\n{}\n",
				available_packages
					.iter()
					.map(|(i, p)| format!("{} {}", i, p))
					.collect::<Vec<String>>()
					.join("\n")
			),
		);

		// TODO: Some kind of prompt
		let mut input: String = String::new();
		zerr!(
			std::io::stdin().read_line(&mut input),
			"Cannot read input: "
		);

		for i in input.trim().split_ascii_whitespace() {
			match i.parse::<usize>() {
				Ok(v) => {
					if available_packages.contains_key(&v) {
						cfg.packages.insert(
							available_packages
								.get(&v)
								.unwrap()
								.to_owned(),
						);
					}
				},
				_ => {},
			}
		}
	} else {
		return Err(ZeusError::new(
			"No packages specified. See --help!",
		));
	}

	#[cfg(debug_assertions)]
	logger.v(Level::Debug, format!("{:?}", &cfg));

	/*

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
	logger.v(
		Level::Debug,
		format!("should_create = {:?}", should_create),
	);

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

	let opts =
		StartContainerOptions::<String> { ..Default::default() };

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
		ctrlc::set_handler(move || tx
			.send(())
			.expect("Cannot send signal")),
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
					.kill_container(
						&cfg.name,
						Some(KillContainerOptions {
							signal: "SIGKILL"
						})
					)
					.await,
				"Cannot kill builder: "
			);
		}

		print!("{}", zerr!(res, "Error displaying builder logs: "));
	} */

	Ok(())
}

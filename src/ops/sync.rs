use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc::channel;

use bollard::container::{
	AttachContainerOptions, AttachContainerResults,
	KillContainerOptions, StartContainerOptions,
};

use futures::StreamExt;

use ctrlc;

use crate::log_error;
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

		let packages: Vec<String> = zerr!(
			fs::read_dir(&cfg.builddir),
			"fs",
			&format!("Cannot list {}", &cfg.builddir)
		)
		.filter_map(|x| x.ok())
		.filter(|x| x.path().is_dir())
		.map(|x| x.file_name().into_string())
		.filter_map(|x| x.ok())
		.collect();

		match logger.question(
			"Choose which packages to upgrade:",
			packages.iter().map(|x| x.as_str()).collect(),
			"all",
			4,
		)? {
			None => {
				for package in packages {
					cfg.packages.insert(package);
				}
			},
			Some(answer) => {
				for package in answer {
					cfg.packages.insert(package.to_owned());
				}
			},
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

	if !logger.yes_no_question(
		match cfg.upgrade {
			true => {
				"Are you sure you want to upgrade these packages?"
			},
			false => "Are you sure you want to build these packages?",
		},
		true,
	)? {
		log_error!(logger, "zeus", "Aborting...");
		return Ok(());
	}

	let socket_path = format!("{}/zeus.sock", &cfg.builddir);

	let listener = zerr!(
		LocalListener::new(Path::new(&socket_path), Some(0o666)),
		"unix",
		format!("Cannot listen on socket {}", &socket_path)
	);

	let opts =
		StartContainerOptions::<String> { ..Default::default() };

	zerr!(
		docker.start_container(&cfg.name, Some(opts)).await,
		"docker",
		"Error starting builder"
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

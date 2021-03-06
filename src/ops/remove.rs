use std::io::Write;
use std::path::Path;
use std::sync::mpsc::channel;

use bollard::container::{
	AttachContainerOptions, AttachContainerResults,
	KillContainerOptions, StartContainerOptions,
};

use futures::StreamExt;

use crate::ops::prelude::*;
use crate::util::LocalListener;

pub async fn remove(
	logger: &Logger,
	docker: Docker,
	cfg: &mut config::AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	cfg.name = args.value_of("name").unwrap().to_owned();

	cfg.packages = args
		.values_of("packages")
		.unwrap_or_default()
		.map(|x| x.to_owned())
		.collect();

	if cfg.packages.is_empty() {
		return Err(ZeusError::new(
			"zeus".to_owned(),
			"No packages specified.".to_owned(),
		));
	}

	log_info!(
		logger,
		"zeus",
		"Removing: {}",
		cfg.packages
			.iter()
			.map(|x| x.as_str())
			.collect::<Vec<&str>>()
			.join(" ")
	);

	cfg.remove = true;

	if !terminal::yes_no_question(
		"Are you sure you want to remove these packages?",
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

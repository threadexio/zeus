use std::fs;
use std::io::prelude::*;

use bollard::container::RemoveContainerOptions;
use bollard::image::BuildImageOptions;
use bollard::Docker;

use clap::ArgMatches;

use futures::StreamExt;

use crate::ops::prelude::*;

pub async fn build(
	logger: &Logger,
	docker: Docker,
	cfg: &mut config::AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	cfg.archive = args.value_of("archive").unwrap().to_owned();
	cfg.dockerfile = args.value_of("dockerfile").unwrap().to_owned();
	cfg.image = args.value_of("image").unwrap().to_owned();
	cfg.name = args.value_of("name").unwrap().to_owned();

	let mut file = zerr!(
		fs::File::open(&cfg.archive),
		"system",
		format!("Cannot open image {}", &cfg.archive)
	);

	let mut contents: Vec<u8> = vec![];
	zerr!(
		file.read_to_end(&mut contents),
		"system",
		format!("Cannot read image {}", &cfg.archive)
	);

	log_info!(logger, "docker", "Starting builder...");

	let opts = BuildImageOptions {
		dockerfile: cfg.dockerfile.as_str(),
		t: cfg.image.as_str(),
		nocache: cfg.force,
		pull: true,
		rm: true,
		..Default::default()
	};

	let mut stream =
		docker.build_image(opts, None, Some(contents.into()));
	while let Some(r) = stream.next().await {
		let build_info = zerr!(r, "docker", "Error during build");

		if let Some(e) = build_info.error {
			return Err(ZeusError::new(
				"docker".to_owned(),
				format!("Error during build: {}", e),
			));
		}

		if let Some(msg) = build_info.stream {
			let msg = msg.trim_end();

			if msg != "" {
				println!("{}", msg)
			}
		}
	}

	log_info!(logger, "docker", "Removing old builder...");

	match docker
		.remove_container(
			"zeus-builder",
			Some(RemoveContainerOptions {
				force: true,
				link: false,
				v: true,
			}),
		)
		.await
	{
		Ok(_) => {},
		Err(e) => {
			log_warn!(
				logger,
				"docker",
				"Cannot remove old builder: {}",
				e
			);
		},
	}

	Ok(())
}

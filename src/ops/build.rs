use std::fs;
use std::io::prelude::*;

use bollard::container::RemoveContainerOptions;
use bollard::image::BuildImageOptions;
use bollard::Docker;

use bollard::container::{Config, CreateContainerOptions};
use bollard::models::{
	HostConfig, Mount, MountBindOptions,
	MountBindOptionsPropagationEnum, MountTypeEnum,
};

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

	let opts = CreateContainerOptions { name: &cfg.name };

	let config =
		Config {
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
		"Error creating new builder"
	);

	Ok(())
}

use crate::config;
use crate::error::{zerr, Result, ZeusError};
use crate::log::{self, Level};

use bollard::container::RemoveContainerOptions;
use bollard::image::BuildImageOptions;
use bollard::Docker;

use futures::StreamExt;

use std::fs::File;
use std::io::prelude::*;

pub async fn build(logger: &mut log::Logger, docker: Docker, cfg: config::AppConfig) -> Result<()> {
	logger.v(
		Level::Verbose,
		format!("Builder image archive: {}", &cfg.archive),
	);

	let mut file = match File::open(&cfg.archive) {
		Ok(v) => v,
		Err(e) => {
			return Err(ZeusError::new(format!("Cannot open image archive: {}", e)));
		}
	};

	let mut contents: Vec<u8> = vec![];
	match file.read_to_end(&mut contents) {
		Ok(_) => {}
		Err(e) => {
			return Err(ZeusError::new(format!("Cannot read image archive: {}", e)));
		}
	}

	logger.v(Level::Info, "Starting builder...");

	let opts = BuildImageOptions {
		dockerfile: cfg.dockerfile,
		t: cfg.image,
		nocache: cfg.force,
		pull: true,
		rm: true,
		..Default::default()
	};

	let mut stream = docker.build_image(opts, None, Some(contents.into()));
	while let Some(r) = stream.next().await {
		let build_info = zerr!(r, "Error during build: ");

		if let Some(e) = build_info.error {
			return Err(ZeusError::new(format!("Error during build: {}", e)));
		}

		if let Some(msg) = build_info.stream {
			let msg = msg.trim_end();

			if msg != "" {
				logger.v(Level::Info, msg);
			}
		}
	}

	logger.v(Level::Verbose, "Removing old builder...");

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
		Ok(_) => {}
		Err(e) => {
			logger.v(Level::Warn, format!("Cannot remove old builder: {}", e));
		}
	}

	Ok(())
}

use crate::config;
use crate::error::ZeusError;
use crate::log::{self, Level};

use bollard::container::RemoveContainerOptions;
use bollard::image::BuildImageOptions;
use bollard::Docker;

use futures::StreamExt;

use std::fs::File;
use std::io::prelude::*;

pub async fn build(
    logger: &mut log::Logger,
    docker: Docker,
    cfg: config::Config,
) -> Result<(), ZeusError> {
    logger.v(
        Level::Verbose,
        config::PROGRAM_NAME,
        format!("Builder image archive: {}", &cfg.builder.archive),
    );

    let mut file = match File::open(&cfg.builder.archive) {
        Ok(v) => v,
        Err(e) => {
            return Err(ZeusError::new(
                "filesystem",
                format!("Cannot open image archive: {}", e),
            ));
        }
    };

    let mut contents: Vec<u8> = vec![];
    match file.read_to_end(&mut contents) {
        Ok(_) => {}
        Err(e) => {
            return Err(ZeusError::new(
                "filesystem",
                format!("Cannot read image archive: {}", e),
            ));
        }
    }

    logger.v(Level::Info, "docker", "Starting builder...\n");

    let opts = BuildImageOptions {
        dockerfile: cfg.builder.dockerfile,
        t: cfg.builder.image,
        nocache: cfg.force,
        pull: true,
        rm: true,
        ..Default::default()
    };

    let mut stream = docker.build_image(opts, None, Some(contents.into()));
    while let Some(r) = stream.next().await {
        match r {
            Err(e) => {
                return Err(ZeusError::new(
                    "docker",
                    format!("Error during build: {}", e),
                ));
            }
            Ok(v) => {
                if let Some(e) = v.error {
                    return Err(ZeusError::new(
                        "docker",
                        format!("Error during build: {}", e),
                    ));
                }

                if let Some(msg) = v.stream {
                    let msg = msg.trim_end();

                    if msg != "" {
                        logger.v(Level::Info, "builder", msg);
                    }
                }
            }
        }
    }

    logger.v(Level::Verbose, "docker", "Removing old builder...");

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
            logger.v(
                Level::Warn,
                "docker",
                format!("Cannot remove old builder: {}", e),
            );
        }
    }

    Ok(())
}

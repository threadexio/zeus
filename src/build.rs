use crate::config;
use crate::log::{self, Level};

use bollard::image::{BuildImageOptions, ListImagesOptions};
use bollard::Docker;

use futures::StreamExt;

use std::fs::File;
use std::io::Read;
use std::process::exit;

pub async fn build(mut logger: log::Logger, docker: Docker, cfg: config::Config) {
    if !cfg.force {
        let opts = ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        };

        match docker.list_images(Some(opts)).await {
            Err(e) => {
                logger.v(
                    Level::Error,
                    "docker",
                    format!("Cannot query available images: {}", e),
                );
                exit(1);
            }
            Ok(v) => {
                for r in v {
                    for name in r.repo_tags {
                        if cfg.builder_image == name {
                            logger.v(Level::Warn, "builder", format!("Other image found, refusing to overwrite. Use --force to override this!"));
                            exit(1);
                        }
                    }
                }
            }
        };
    }

    let opts = BuildImageOptions {
        dockerfile: cfg.builder_dockerfile,
        t: cfg.builder_image,
        nocache: cfg.force,
        pull: cfg.force,
        rm: true,
        ..Default::default()
    };

    let mut file = match File::open(&cfg.builder_archive) {
        Ok(v) => v,
        Err(e) => {
            logger.v(
                Level::Error,
                "filesystem",
                format!("Cannot open image archive: {}", e),
            );
            exit(1);
        }
    };

    let mut contents: Vec<u8> = vec![];
    match file.read_to_end(&mut contents) {
        Err(e) => {
            logger.v(
                Level::Error,
                "filesystem",
                format!("Cannot read image archive: {}", e),
            );
            exit(1);
        }
        _ => {}
    }

    logger.v(Level::Info, "docker", "Starting builder...\n");

    let mut stream = docker.build_image(opts, None, Some(contents.into()));
    while let Some(r) = stream.next().await {
        match r {
            Err(e) => {
                logger.v(
                    Level::Error,
                    "docker",
                    format!("Fatal error during build: {}", e),
                );
                exit(1);
            }
            Ok(v) => {
                if let Some(err) = v.error {
                    println!("");
                    logger.v(
                        Level::Error,
                        "builder",
                        format!("Error during image build: {}", err),
                    );
                    exit(1);
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

    println!("");
    logger.v(Level::Success, "builder", "Finished successfully");
}

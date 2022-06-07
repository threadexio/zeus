//mod build;
//mod misc;
//mod query;
//mod remove;
mod sync;

use crate::config::AppConfig;
use crate::error::{zerr, Result, ZeusError};
use crate::log::Logger;
use crate::util::Lockfile;

use bollard::Docker;
use clap::ArgMatches;

use std::path;

fn init_docker(cfg: &mut AppConfig) -> Result<()> {
	cfg.docker = match Docker::connect_with_local_defaults() {
		Ok(v) => Some(v),
		Err(e) => {
			return Err(ZeusError::new(format!(
				"Cannot connect to the docker daemon: {}",
				e
			)))
		},
	};

	Ok(())
}

pub async fn run_operation(
	name: &str,
	logger: &mut Logger,
	cfg: &mut AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	let lockfile = zerr!(
		Lockfile::new(path::Path::new(&format!(
			"{}/zeus.lock",
			&cfg.builddir
		))),
		"Cannot create lockfile: "
	);

	match name {
		//		"build" => {
		//			lockfile.lock()?;
		//			init_docker(cfg)?;
		//			build::build(logger, cfg, args).await
		//		},
		//		"remove" => {
		//			lockfile.lock()?;
		//			init_docker(cfg)?;
		//			remove::remove(logger, cfg, args).await
		//		},
		"sync" => {
			lockfile.lock()?;
			init_docker(cfg)?;
			sync::sync(logger, cfg, args).await
		},
		//		"misc" => misc::misc(logger, cfg, args).await,
		//		"query" => query::query(logger, cfg, args).await,
		_ => Err(ZeusError::new("No such operation")),
	}
}

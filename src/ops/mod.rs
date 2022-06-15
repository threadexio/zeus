use std::path;

use bollard::Docker;
use clap::ArgMatches;

use crate::config::AppConfig;
use crate::error::{Result, ZeusError};
use crate::log::Logger;
use crate::log_debug;
use crate::util::Lockfile;

mod build;
mod completions;
mod query;
mod remove;
mod sync;

mod prelude {
	pub(crate) use bollard::Docker;
	pub(crate) use clap::ArgMatches;

	pub(crate) use crate::config;
	pub(crate) use crate::error::{zerr, Result, ZeusError};
	pub(crate) use crate::log::Logger;
	pub(crate) use crate::{
		log_debug, log_error, log_info, log_warn,
	};
}

fn init_docker() -> Result<Docker> {
	match Docker::connect_with_local_defaults() {
		Ok(v) => return Ok(v),
		Err(e) => {
			return Err(ZeusError::new(
				"docker".to_owned(),
				format!("Cannot connect to the docker daemon: {}", e),
			))
		},
	};
}

pub async fn run_operation(
	name: &str,
	logger: &Logger,
	cfg: &mut AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	let lockfile = Lockfile::new(path::Path::new(&format!(
		"{}/zeus.lock",
		&cfg.builddir
	)))?;

	log_debug!(logger, "pre-op config", "{:?}", cfg);

	match name {
		"build" => {
			lockfile.lock()?;
			build::build(logger, init_docker()?, cfg, args).await
		},
		"remove" => {
			lockfile.lock()?;
			remove::remove(logger, init_docker()?, cfg, args).await
		},
		"sync" => {
			lockfile.lock()?;
			sync::sync(logger, init_docker()?, cfg, args).await
		},
		"completions" => {
			completions::completions(logger, cfg, args).await
		},
		"query" => query::query(cfg, args).await,
		_ => Err(ZeusError::new(
			"zeus".to_owned(),
			"No such operation".to_owned(),
		)),
	}
}

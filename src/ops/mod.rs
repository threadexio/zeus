use std::path;

use bollard::Docker;
use clap::ArgMatches;

use crate::config::{AppConfig, Operation};
use crate::debug;
use crate::error::{Result, ZeusError};
use crate::term::Terminal;
use crate::util::Lockfile;

mod build;
mod completions;
mod query;
mod remove;
mod sync;

mod prelude {
	pub use crate::config;

	// Error handling
	pub use crate::error::{Result, ZeusError};
	pub use crate::zerr;

	pub use crate::term::Terminal;

	// Logging
	pub use crate::log::Logger;
	pub use crate::{debug, error, info, warn};

	// Extras
	pub use bollard::Docker;
	pub use clap::ArgMatches;
	pub use colored::Colorize;
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
	term: &mut Terminal,
	cfg: &mut AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	let lockfile = Lockfile::new(path::Path::new(&format!(
		"{}/zeus.lock",
		&cfg.builddir
	)))?;

	debug!(term.log, "pre-op config", "{:?}", cfg);

	match name {
		"build" => {
			lockfile.lock()?;
			build::build(&term, init_docker()?, cfg, args).await
		},
		"remove" => {
			lockfile.lock()?;
			cfg.operation = Operation::Remove;
			remove::remove(term, init_docker()?, cfg, args).await
		},
		"sync" => {
			lockfile.lock()?;
			cfg.operation = Operation::Sync;
			sync::sync(term, init_docker()?, cfg, args).await
		},
		"query" => query::query(cfg, args).await,
		"completions" => completions::completions(args).await,
		_ => Err(ZeusError::new(
			"zeus".to_owned(),
			"No such operation".to_owned(),
		)),
	}
}

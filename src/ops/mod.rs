use std::env;
use std::path;

use crate::config::Operation;
use crate::lock::Lockfile;
use crate::machine::manager::RuntimeManager;

mod build;
mod completions;
mod query;
mod remove;
mod sync;

mod prelude {
	pub use crate::config::AppConfig;

	// Error handling
	pub use crate::error::{Result, ZeusError};
	pub use crate::zerr;

	pub use crate::term::Terminal;

	// Logging
	pub use crate::log::Logger;
	pub use crate::{debug, error, info, warn};

	// Extras
	pub use crate::machine::Runtime;
	pub use clap::ArgMatches;
	pub use colored::Colorize;
}

use prelude::*;

fn get_runtime<'a>(
	cfg: &AppConfig,
	rt_manager: &'a mut RuntimeManager,
) -> Result<&'a mut Runtime> {
	zerr!(
		env::set_current_dir(crate::config::DATA_DIR),
		"system",
		"Cannot change directory to {}:",
		crate::config::DATA_DIR
	);

	Ok(rt_manager
		.load(format!(
			"{}/librt_{}.so",
			cfg.runtime_dir, cfg.runtime
		))?
		.as_mut())
}

fn get_lock(lockfile: &Lockfile) -> Result<()> {
	Ok(zerr!(lockfile.lock(), "system", "Cannot obtain lock"))
}

pub fn run_operation(
	name: &str,
	term: &mut Terminal,
	mut cfg: AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	let lockfile = zerr!(
		Lockfile::new(path::Path::new(&format!(
			"{}/zeus.lock",
			&cfg.build_dir
		))),
		"system",
		"Cannot create lock"
	);

	let mut rt_manager = RuntimeManager::new();

	debug!(term.log, "pre-op config", "{:?}", cfg);

	match name {
		"build" => {
			get_lock(&lockfile)?;
			build::build(
				term,
				get_runtime(&cfg, &mut rt_manager)?,
				cfg,
				args,
			)
		},
		"remove" => {
			get_lock(&lockfile)?;
			cfg.operation = Operation::Remove;
			remove::remove(
				term,
				get_runtime(&cfg, &mut rt_manager)?,
				cfg,
				args,
			)
		},
		"sync" => {
			get_lock(&lockfile)?;
			cfg.operation = Operation::Sync;
			sync::sync(
				term,
				get_runtime(&cfg, &mut rt_manager)?,
				cfg,
				args,
			)
		},
		"query" => query::query(term, cfg, args),
		"completions" => completions::completions(args),
		_ => Err(ZeusError::new(
			"zeus".to_owned(),
			"No such operation".to_owned(),
		)),
	}
}

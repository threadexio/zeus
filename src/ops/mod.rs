use std::env;
use std::path::Path;

use crate::config::Operation;
use crate::lock::Lockfile;
use crate::machine::manager::RuntimeManager;
use crate::message::Message;
use crate::unix::LocalListener;

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

pub fn start_builder(
	term: &mut Terminal,
	runtime: &mut Runtime,
	cfg: AppConfig,
) -> Result<Vec<String>> {
	if !runtime.list_machines()?.iter().any(|x| x == &cfg.machine) {
		return Err(ZeusError::new(
			"zeus".to_owned(),
			"No builder machine found.".to_owned(),
		));
	}

	let socket_path = format!("{}/zeus.sock", &cfg.build_dir);
	let listener = zerr!(
		LocalListener::new(Path::new(&socket_path), 0o666),
		"unix",
		"Cannot listen on socket {}",
		&socket_path
	);

	info!(term.log, "zeus", "Starting builder...");
	runtime.start_machine(&cfg.machine)?;

	debug!(term.log, "unix", "Waiting for builder to connect...");
	let (mut channel, _) = zerr!(
		listener.accept(),
		"unix",
		"Cannot open communication stream with builder"
	);

	debug!(term.log, "zeus", "Sending config to builder...");
	channel.send(Message::Config(cfg))?;

	debug!(term.log, "zeus", "Entering main event loop...");
	loop {
		match channel.recv()? {
			Message::Success(pkgs) => {
				return Ok(pkgs);
			},
			Message::Failure(error) => {
				return Err(ZeusError::new(
					"builder".to_string(),
					error,
				))
			},
			_ => {},
		}
	}
}

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

fn get_lock(
	lockfile: &mut Option<Lockfile>,
	cfg: &AppConfig,
) -> Result<()> {
	if lockfile.is_none() {
		*lockfile = Some(zerr!(
			Lockfile::new(Path::new(&format!(
				"{}/.zeus.lock",
				&cfg.build_dir
			))),
			"system",
			"Cannot create lock"
		));
	}

	Ok(zerr!(
		lockfile.as_ref().unwrap().try_lock(),
		"system",
		"Cannot obtain lock"
	))
}

pub fn run_operation(
	term: &mut Terminal,
	mut cfg: AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	let mut lockfile: Option<Lockfile> = None;

	let mut rt_manager = RuntimeManager::new();

	debug!(term.log, "pre-op config", "{:?}", cfg);

	match cfg.operation {
		Operation::Build => {
			get_lock(&mut lockfile, &cfg)?;
			build::build(
				term,
				get_runtime(&cfg, &mut rt_manager)?,
				cfg,
				args,
			)
		},
		Operation::Remove => {
			get_lock(&mut lockfile, &cfg)?;
			cfg.operation = Operation::Remove;
			remove::remove(
				term,
				get_runtime(&cfg, &mut rt_manager)?,
				cfg,
				args,
			)
		},
		Operation::Sync => {
			get_lock(&mut lockfile, &cfg)?;
			cfg.operation = Operation::Sync;
			sync::sync(
				term,
				get_runtime(&cfg, &mut rt_manager)?,
				cfg,
				args,
			)
		},
		Operation::Query => query::query(term, cfg, args),
		Operation::Completions => completions::completions(args),
		_ => Err(ZeusError::new(
			"zeus".to_owned(),
			"No such operation".to_owned(),
		)),
	}
}

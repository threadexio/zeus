use std::env;
use std::path::Path;
use std::thread;

use crate::config::Operation;
use crate::lock::Lockfile;
use crate::machine::manager::RuntimeManager;
use crate::message::Message;
use crate::unix::LocalListener;

mod build;
mod completions;
mod query;
mod remove;
mod runtime;
mod sync;

mod prelude {
	pub use crate::config::AppConfig;

	// Error handling
	pub use crate::error::{Result, ZeusError};
	pub use crate::zerr;

	pub use crate::term::Terminal;

	// Logging
	pub use crate::log::Logger;
	pub use crate::{debug, error, info, warning};

	// Extras
	pub use crate::machine::Runtime;
	pub use clap::ArgMatches;
	pub use colored::Colorize;

	pub use super::start_builder;
}

use prelude::*;

pub fn start_builder(
	runtime: &mut Runtime,
	cfg: AppConfig,
) -> Result<Vec<String>> {
	if !runtime.list_machines()?.iter().any(|x| x == &cfg.machine) {
		return Err(ZeusError::new(
			"zeus".to_owned(),
			"No builder machine found.".to_owned(),
		));
	}

	use std::sync::mpsc;
	let (local_tx, local_rx) = mpsc::channel::<()>();

	let cfg1 = cfg.clone();
	let manager_thread =
		thread::spawn(move || -> Result<Vec<String>> {
			let socket_path =
				format!("{}/.zeus.sock", &cfg.build_dir);
			let listener = zerr!(
				LocalListener::new(Path::new(&socket_path), 0o666),
				"unix",
				"Cannot listen on socket {}",
				&socket_path
			);

			// let the main thread continue and start the machine
			local_tx.send(()).unwrap();

			debug!("unix", "Waiting for builder to connect...");
			let (mut tx, mut rx) = zerr!(
				listener.accept(),
				"unix",
				"Cannot open communication stream with builder"
			);

			debug!("zeus", "Sending config to builder...");
			tx.send(Message::Config(cfg1))?;

			debug!("zeus", "Entering main event loop...");
			loop {
				use std::io::ErrorKind;
				match rx.recv() {
					Err(e) if e.kind() == ErrorKind::WouldBlock => {
						continue
					},
					Err(e) => {
						return Err(ZeusError::new(
							"zeus".to_string(),
							format!(
							"Cannot receive message from builder: {}",
							e
						),
						))
					},
					Ok(v) => match v {
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
					},
				};
			}
		});

	// block until the manager thread is ready
	match local_rx.recv() {
		// this is a RecvErr, which means the manager exited prematurely
		Err(_) => {
			return manager_thread.join().unwrap();
		},
		_ => {},
	}

	info!("zeus", "Starting builder...");
	runtime.start_machine(&cfg.machine)?;

	return manager_thread.join().unwrap();
}

fn get_runtime<'a>(
	cfg: &AppConfig,
	rt_manager: &'a mut RuntimeManager,
) -> Result<&'a mut Runtime> {
	let runtime = rt_manager
		.load(format!(
			"{}/librt_{}.so",
			cfg.runtime_dir, cfg.runtime
		))?
		.as_mut();

	zerr!(
		env::set_current_dir(crate::config::defaults::DATA_DIR),
		"system",
		"Cannot change directory to {}:",
		crate::config::defaults::DATA_DIR
	);

	Ok(runtime)
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
	cfg: AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	let mut lockfile: Option<Lockfile> = None;

	let mut rt_manager = RuntimeManager::new();

	debug!("pre-op config", "{:?}", cfg);

	match cfg.operation {
		Operation::Build => {
			get_lock(&mut lockfile, &cfg)?;
			build::build(
				get_runtime(&cfg, &mut rt_manager)?,
				cfg,
				args,
			)
		},
		Operation::Remove => {
			get_lock(&mut lockfile, &cfg)?;
			remove::remove(
				term,
				get_runtime(&cfg, &mut rt_manager)?,
				cfg,
				args,
			)
		},
		Operation::Sync => {
			get_lock(&mut lockfile, &cfg)?;
			sync::sync(
				term,
				get_runtime(&cfg, &mut rt_manager)?,
				cfg,
				args,
			)
		},
		Operation::Runtime => {
			get_lock(&mut lockfile, &cfg)?;
			runtime::runtime(term, &mut rt_manager, cfg, args)
		},
		Operation::Query => query::query(term, cfg, args),
		Operation::Completions => completions::completions(args),
		Operation::None => Err(ZeusError::new(
			"zeus".to_owned(),
			"No such operation".to_owned(),
		)),
	}
}

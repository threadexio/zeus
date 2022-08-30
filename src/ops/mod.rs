use std::env;
use std::path::Path;
use std::thread;

use crate::unix::LocalListener;

mod build;
mod completions;
mod query;
mod remove;
mod runtime;
mod sync;

mod prelude {
	pub use crate::term::Terminal;

	pub use crate::cache::BuildCache;

	pub use crate::aur::Package;

	pub use crate::message::PackageMeta;

	pub use crate::machine::{manager::RuntimeManager, Runtime};

	// Error handling
	pub use crate::error::{Error, Result};
	pub(crate) use error::{err, other};

	pub use crate::config::{
		constants, BuildOptions, CompletionOptions, Config,
		GlobalOptions, Operation, QueryOptions, RemoveOptions,
		RuntimeOptions, SyncOptions,
	};

	// Logging
	pub use crate::log::Logger;
	pub use crate::{debug, error, info, warn};

	// Extras
	pub use colored::Colorize;

	pub use super::start_builder;
}

use prelude::*;

macro_rules! get_runtime {
	($rt_manager:expr, $c:expr) => {{
		err!(
			env::set_current_dir(constants::defaults::DATA_DIR),
			"Cannot change directory to {}",
			constants::defaults::DATA_DIR
		);

		$rt_manager
			.load(format!(
				"{}/librt_{}.so",
				$c.runtime_dir, $c.runtime
			))?
			.as_mut()
	}};
}

pub fn run_operation(term: &mut Terminal, cfg: Config) -> Result<()> {
	let mut build_cache =
		BuildCache::new(&cfg.global_opts.build_dir)?;

	let mut rt_manager = RuntimeManager::new();

	let mut gopts = cfg.global_opts;

	match cfg.operation {
		Operation::Build(v) => {
			build_cache.lock()?;
			build::build(
				get_runtime!(rt_manager, gopts),
				&mut gopts,
				v,
			)
		},
		Operation::Remove(v) => {
			build_cache.lock()?;
			remove::remove(
				term,
				get_runtime!(rt_manager, gopts),
				&mut build_cache,
				&mut gopts,
				v,
			)
		},
		Operation::Sync(v) => {
			build_cache.lock()?;
			sync::sync(
				term,
				get_runtime!(rt_manager, gopts),
				&mut build_cache,
				&mut gopts,
				v,
			)
		},
		Operation::Runtime(v) => {
			build_cache.lock()?;
			runtime::runtime(term, &mut rt_manager, &mut gopts, v)
		},
		Operation::Query(v) => query::query(term, &mut gopts, v),
		Operation::Completions(v) => completions::completions(v),
	}
}

pub fn start_builder(
	runtime: &mut Runtime,
	build_cache: &BuildCache,
	cfg: &Config,
	machine_name: &str,
) -> Result<Vec<PackageMeta>> {
	use crate::message::Message;

	if !runtime
		.list_machines()
		.map_err(|x| other!("{}", x))?
		.iter()
		.any(|x| x == machine_name)
	{
		return Err(other!("No builder machine found."));
	}

	use std::sync::mpsc;
	let (local_tx, local_rx) = mpsc::channel::<()>();

	let cfg1 = cfg.clone();
	let build_dir = build_cache.path().display().to_string();

	let manager_thread = thread::spawn(move || {
		let socket_path = format!("{}/.zeus.sock", build_dir);
		let listener = err!(
			LocalListener::new(Path::new(&socket_path), 0o666),
			"Cannot listen on socket {}",
			&socket_path
		);

		// let the main thread continue and start the machine
		local_tx.send(()).unwrap();

		let (mut tx, mut rx) = err!(
			listener.accept(),
			"Cannot open communication stream with builder"
		);

		tx.send(Message::Config(cfg1))?;

		let mut packages: Vec<PackageMeta> = vec![];

		loop {
			use std::io::ErrorKind;
			match rx.recv() {
				Err(e) if e.kind() == ErrorKind::WouldBlock => {
					continue
				},
				Err(e) => {
					return Err(other!(
						"Cannot receive message from builder: {}",
						e
					))
				},
				Ok(v) => match v {
					Message::PackageMeta(pkg) => packages.push(pkg),
					Message::Success => {
						return Ok(packages);
					},
					Message::Failure(error) => {
						return Err(other!("{}", error))
					},
					_ => {
						panic!("received unexpected message: {:?}", v)
					},
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

	info!("Starting builder...");
	runtime
		.start_machine(machine_name)
		.map_err(|x| other!("{}", x))?;

	return manager_thread.join().unwrap();
}

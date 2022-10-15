mod build;
mod completions;
mod query;
mod remove;
mod runtime;
mod sync;

pub mod prelude {
	pub use crate::machine::Runtime;

	pub use crate::config::{
		BuildOptions, CompletionOptions, Config, GlobalOptions,
		Operation, QueryOptions, RemoveOptions, RuntimeOptions,
		SyncOptions,
	};

	pub use crate::error::*;
	pub use crate::package::*;

	// Logging macros
	pub use crate::{debug, error, info, warn};

	pub(crate) use crate::constants;
}

use prelude::*;

use std::path::Path;

fn load_runtime(opts: &GlobalOptions) -> Result<Runtime> {
	std::env::set_current_dir(constants::DATA_DIR).context(
		format!(
			"Unable to change directory to {}",
			constants::DATA_DIR
		),
	)?;

	let path = Path::new(&opts.runtime_dir)
		.join(format!("librt_{}.so", &opts.runtime));

	Runtime::load(&path, opts).context("Unable to load runtime")
}

fn require_lock(
	pstore: &mut PackageStore,
) -> Result<&mut PackageStore> {
	pstore.lock().context("Unable to lock package cache")?;
	Ok(pstore)
}

pub fn run_operation(cfg: Config) -> Result<()> {
	let opts = cfg.global_opts;

	let mut pstore = PackageStore::new(&opts.build_dir)
		.context("Unable to create package store")?;

	match cfg.operation {
		Operation::Build(v) => {
			require_lock(&mut pstore)?;
			let mut runtime = load_runtime(&opts)?;
			build::build(&mut runtime, opts, v)
		},
		Operation::Remove(v) => {
			let mut runtime = load_runtime(&opts)?;
			remove::remove(
				&mut runtime,
				require_lock(&mut pstore)?,
				opts,
				v,
			)
		},
		Operation::Sync(v) => sync::sync(
			load_runtime(&opts)?,
			require_lock(&mut pstore)?,
			opts,
			v,
		),
		Operation::Runtime(v) => {
			require_lock(&mut pstore)?;
			runtime::runtime(opts, v)
		},
		Operation::Query(v) => query::query(opts, v),
		Operation::Completions(v) => completions::completions(v),
	}
}

pub(self) fn start_builder(
	_runtime: &mut Runtime,
	pstore: &mut PackageStore,
	opts: &GlobalOptions,
	op: Operation,
) -> Result<()> {
	// 1. Establish communication with builder
	//		- Listen on /var/cache/aur/.zeus.sock
	//
	// 2. Send configuration
	//		- Send `opts`
	//		- Send current operation `crate::config::Operation` with opts
	//
	// 3. Wait for builder to finish
	//		- Block until further messages
	//
	// 4. Receive data
	//		- Receive failed packages
	//		- Receive built packages
	//		- Receive package archives
	//
	// 5. Terminate builder
	//		- Stop builder gracefully or kill it
	//
	// 6. Return data

	use crate::ipc::{Listener, Message};
	use std::thread;

	let config = Config { global_opts: opts.clone(), operation: op };

	let path = pstore.root().join(".zeus.sock");
	let handle = thread::Builder::new()
		.spawn(move || -> Result<()> {
			let mut ipc = Listener::new(path)?;

			ipc.send(Message::Init(config))
				.context("failed to initialize builder")?;

			loop {
				match ipc.recv().context("failed to recv message")? {
					Message::End => {
						debug!(
							"End message received. Exiting loop..."
						);
						return Ok(());
					},
					m => debug!(
						"Received message without handler: {:?}",
						m
					),
				}
			}
		})
		.context("failed to spawn thread")?;

	//runtime
	//	.start_machine(&opts.machine_name)
	//	.context("unable to start builder")?;

	handle.join().unwrap()
}

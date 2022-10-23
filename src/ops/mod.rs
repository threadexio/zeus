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
		.context("Unable to initialize build cache")?;

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
		Operation::Query(v) => {
			query::query(require_lock(&mut pstore)?, opts, v)
		},
		Operation::Completions(v) => completions::completions(v),
	}
}

pub(self) fn start_builder(
	runtime: &mut Runtime,
	pstore: &mut PackageStore,
	opts: &GlobalOptions,
	op: Operation,
) -> Result<Vec<Package>> {
	/*
	! IMPORTANT: Do not use anything that will write to stdout or stderr here,
	!            because we expect the runtime to have attached the builder to
	!            them and doing so might mess up the output.
	*/

	use crate::ipc::{Listener, Message};
	use std::thread;

	let config = Config { global_opts: opts.clone(), operation: op };

	let path = pstore.root().join(".zeus.sock");
	let handle = thread::Builder::new()
		.spawn(move || -> Result<Vec<Package>> {
			debug!("Setting up communication channel...");
			let mut ipc = Listener::new(path)?;

			ipc.send(Message::Init(config))
				.context("failed to initialize builder")?;

			loop {
				match ipc.recv().context("failed to recv message")? {
					Message::End(pkgs) => {
						return Ok(pkgs);
					},
					_ => {},
				}
			}
		})
		.context("failed to spawn manager thread")?;

	debug!("Starting builder...");
	runtime
		.start_machine(&opts.machine_name)
		.context("failed to start builder")?;

	debug!("Builder exited...");

	handle.join().unwrap()
}

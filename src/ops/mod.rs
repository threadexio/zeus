mod build;
mod completions;
mod query;
mod remove;
mod runtime;
mod sync;

pub mod prelude {
	pub use crate::error::*;

	pub use crate::machine::Runtime;

	pub use crate::config::{
		BuildOptions, CompletionOptions, Config, GlobalOptions,
		QueryOptions, RemoveOptions, RuntimeOptions, SyncOptions,
	};

	pub use crate::package::*;

	pub use crate::{debug, error, info, warn};

	//pub(crate) use crate::term;
}

use prelude::*;

use std::path::Path;

fn load_runtime(opts: &GlobalOptions) -> Result<Runtime> {
	std::env::set_current_dir(crate::config::constants::DATA_DIR)
		.context(format!(
			"Unable to change directory to {}",
			crate::config::constants::DATA_DIR
		))?;

	let path = Path::new(&opts.runtime_dir)
		.join(format!("librt_{}.so", &opts.runtime));

	Runtime::load(&path, opts).context("Unable to load runtime")
}

fn require_lock(pstore: &mut PackageStore) -> Result<()> {
	pstore.lock().context("Unable to lock package cache")
}

pub fn run_operation(cfg: Config) -> Result<()> {
	let opts = cfg.global_opts;

	let mut pstore = PackageStore::new(&opts.build_dir)
		.context("Unable to create package store")?;

	use crate::config::Operation;
	match cfg.operation {
		Operation::Build(v) => {
			require_lock(&mut pstore)?;
			let mut runtime = load_runtime(&opts)?;
			build::build(&mut runtime, opts, v)
		},
		Operation::Remove(v) => {
			require_lock(&mut pstore)?;
			remove::remove(
				//&mut load_runtime(&opts)?,
				&mut pstore,
				opts,
				v,
			)
		},
		Operation::Sync(v) => {
			require_lock(&mut pstore)?;
			sync::sync(&mut pstore, opts, v)
		},
		Operation::Runtime(v) => {
			require_lock(&mut pstore)?;
			runtime::runtime(opts, v)
		},
		Operation::Query(v) => query::query(opts, v),
		Operation::Completions(v) => completions::completions(v),
	}
}

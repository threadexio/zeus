use std::env;
use std::path::{Path, PathBuf};

mod build;
mod completions;
mod query;
mod remove;
mod runtime;
mod sync;

mod prelude {
	pub use anyhow::{anyhow, bail, Context, Result};
	pub use colored::Colorize;

	pub use zeus_aur::Aur;
	pub use zeus_db::Db;
	pub use zeus_rt::Runtime;
	pub use zeus_term::Terminal;
	pub use zeus_types::{
		constants, AppConfig, BuildConfig, Color, CompletionsConfig,
		GlobalConfig, Operation, QueryConfig, RemoveConfig,
		RuntimeConfig, SyncConfig,
	};
}

use prelude::*;

pub fn init(term: &mut Terminal) -> Result<()> {
	let AppConfig { global: mut global_config, operation } =
		zeus_types::load()?;

	match global_config.color {
		Color::Always => term.set_color_enabled(true),
		Color::Never => term.set_color_enabled(false),
		_ => {},
	};

	if term.is_interactive() {
		term.set_interactive(!global_config.no_confirm);
	}

	term.debug(format!(
		"Version: {}",
		constants::VERSION.bright_blue()
	));

	global_config.db_key = rand::random();
	term.trace(format!("Database key: {}", global_config.db_key));

	let mut db =
		Db::new(&global_config.build_dir).with_context(|| {
			format!(
				"Unable to initialize database at '{}'",
				&global_config.build_dir.display()
			)
		})?;

	let mut aur =
		Aur::new(&global_config.aur_url, constants::AUR_IDENTITY)
			.context("Unable to initialize AUR client")?;

	let mut init_runtime = || -> Result<Runtime> {
		env::set_current_dir(Path::new(constants::DATA_DIR))
			.with_context(|| {
				format!("Unable to move into {}", constants::DATA_DIR)
			})?;

		let mut rt_path = PathBuf::new();
		rt_path.push(constants::LIB_DIR);
		rt_path.push("runtimes");
		rt_path.push(format!("librt_{}.so", global_config.runtime));

		let mut runtime =
			Runtime::load(&rt_path).with_context(|| {
				format!(
					"Unable to load runtime '{}'",
					rt_path.display()
				)
			})?;

		runtime
			.init(&global_config, term)
			.context("Unable to initialize runtime")?;

		Ok(runtime)
	};

	db.lock(global_config.db_key).with_context(|| {
		format!(
			"Unable to obtain lock on database at '{}'",
			db.path().display()
		)
	})?;

	match operation {
		Operation::Sync(config) => {
			let mut runtime = init_runtime()?;

			sync::sync(
				global_config,
				config,
				term,
				&mut runtime,
				&mut db,
				&mut aur,
			)
		},
		Operation::Remove(config) => {
			let mut runtime = init_runtime()?;

			remove::remove(
				global_config,
				config,
				term,
				&mut runtime,
				&mut db,
			)
		},
		Operation::Build(config) => {
			let mut runtime = init_runtime()?;

			build::build(global_config, config, term, &mut runtime)
		},
		Operation::Query(config) => query::query(
			global_config,
			config,
			term,
			&mut db,
			&mut aur,
		),
		Operation::Runtime(config) => {
			runtime::runtime(global_config, config, term)
		},
		Operation::Completion(config) => {
			completions::completions(global_config, config, term)
		},
	}
}

use zeus_ipc::{Message, Response};

pub(self) fn start_builder(
	global_config: GlobalConfig,
	operation: Message,
	term: &mut Terminal,
	runtime: &mut Runtime,
) -> Result<Response> {
	/*
	! IMPORTANT: Do not use anything that will write to stdout or stderr here,
	!            because we expect the runtime to have attached the builder to
	!            them and doing so might mess up the output.
	*/

	use ::std::thread;

	let builder = thread::Builder::new()
		.spawn({let global_config = global_config.clone(); move || -> Result<Response> {
			use zeus_ipc::Listener;

			let mut ipc =
				Listener::new(global_config.build_dir.join(".zeus.sock"))
					.context("Unable to create listener")?;

			ipc.send(Message::Init(global_config))
				.context("Unable to initialize builder")?;

			ipc.send(operation)
				.context("Unable to send operation to builder")?;

			match ipc.recv().context("Unable to receive data from builder")? {
				Message::Response(res) => Ok(res),
				r => Err(anyhow!("received unexpected response from builder: {r:#?}")),
			}
		}})
		.context("Unable to create builder thread")?;

	runtime
		.start_machine(&global_config, term)
		.context("Unable to start machine")?;

	match builder.join() {
		Ok(v) => v,
		Err(_) => Err(anyhow!("Unable to join builder thread")),
	}
}
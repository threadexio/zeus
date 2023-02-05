use ::std::{env, path::Path};

mod prelude {
	pub(crate) use crate::{
		aur,
		config::{
			self, BuildConfig, CompletionsConfig, GlobalConfig,
			QueryConfig, RemoveConfig, RuntimeConfig, SyncConfig,
		},
		constants, db, ipc,
		log::{self, macros::*},
		runtime::Runtime,
	};

	pub use anyhow::{anyhow, bail, Context, Result};

	pub use colored::Colorize;
}
use prelude::*;

use config::{load, types::Color, AppConfig, Operation};

mod build;
mod completions;
mod query;
mod remove;
mod runtime;
mod sync;

pub fn init() -> Result<()> {
	let AppConfig { global: global_config, operation } = load()?;
	init_global(&global_config)?;

	trace!("global config = {:#?}", &global_config);
	trace!("operation = {:#?}", &operation);

	let mut aur = aur::Aur::new(&global_config.aur_url)
		.context("Unable to initialize AUR client")?;

	let mut db =
		db::Db::new(&global_config.build_dir).with_context(|| {
			format!(
				"Unable to initialize database {}",
				&global_config.build_dir.display()
			)
		})?;

	let get_lock =
		|| db.lock().context("Unable to obtain lock on database");

	let load_runtime = |name: &str| {
		env::set_current_dir(Path::new(constants::DATA_DIR))
			.with_context(|| {
				format!("Unable to move into {}", constants::DATA_DIR)
			})?;

		Runtime::load(
			Path::new(constants::LIB_DIR)
				.join("runtimes")
				.join(format!("librt_{name}.so")),
		)
		.context(format!("Unable to load runtime {name}"))
	};

	match operation {
		Operation::Sync(config) => {
			let mut runtime = load_runtime(&global_config.runtime)?;
			runtime.init(&global_config)?;

			sync::sync(
				global_config,
				config,
				&mut runtime,
				get_lock()?,
				&mut aur,
			)
		},
		Operation::Remove(config) => {
			let mut runtime = load_runtime(&global_config.runtime)?;
			runtime.init(&global_config)?;

			remove::remove(
				global_config,
				config,
				&mut runtime,
				get_lock()?,
			)
		},
		Operation::Build(config) => {
			get_lock()?;

			let mut runtime = load_runtime(&global_config.runtime)?;
			runtime.init(&global_config)?;

			build::build(global_config, config, &mut runtime)
		},
		Operation::Query(config) => {
			query::query(global_config, config, &mut db, &mut aur)
		},
		Operation::Runtime(config) => {
			runtime::runtime(global_config, config)
		},
		Operation::Completion(config) => {
			completions::completions(global_config, config)
		},
	}
}

/// Initialize the environment
fn init_global(config: &GlobalConfig) -> Result<()> {
	match config.color {
		Color::Always => log::set_color_enabled(true),
		Color::Never => log::set_color_enabled(false),
		_ => {},
	};

	set_log_level!(config.log_level.clone());

	debug!("Version: {}", constants::VERSION.bright_blue());

	inquire::set_global_render_config(
		inquire::ui::RenderConfig::default_colored()
			.with_prompt_prefix(
				inquire::ui::Styled::new("=>")
					.with_fg(inquire::ui::Color::LightGreen),
			)
			.with_unselected_checkbox(inquire::ui::Styled::new(" "))
			.with_selected_checkbox(
				inquire::ui::Styled::new("*")
					.with_fg(inquire::ui::Color::LightGreen),
			),
	);

	Ok(())
}

use ipc::{Message, Response};

pub(self) fn start_builder(
	global_config: GlobalConfig,
	operation: Message,
	runtime: &mut Runtime,
) -> Result<Response> {
	/*
	! IMPORTANT: Do not use anything that will write to stdout or stderr here,
	!            because we expect the runtime to have attached the builder to
	!            them and doing so might mess up the output.
	*/

	use ::std::thread;

	let machine_name = global_config.machine_name.clone();

	let builder = thread::Builder::new()
		.spawn(move || -> Result<Response> {
			use crate::ipc::Listener;

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
		})
		.context("Unable to create builder thread")?;

	runtime
		.start_machine(&machine_name)
		.context("Unable to start machine")?;

	debug!("Waiting for builder thread to finish...");
	match builder.join() {
		Ok(v) => v,
		Err(_) => Err(anyhow!("Unable to join builder thread")),
	}
}

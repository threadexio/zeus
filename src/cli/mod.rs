//mod ops;

use ::std::{env, path::Path};

mod prelude {
	pub(crate) use crate::{
		aur,
		config::*,
		constants, db,
		ipc::{Message, Response},
		log::{self, macros::*},
		runtime::Runtime,
	};

	pub use anyhow::{anyhow, bail, Context, Result};

	pub use colored::Colorize;
}

use prelude::*;

mod build;
mod completions;
mod query;
mod remove;
mod runtime;
mod sync;

/// Start the cli
pub fn init() -> Result<()> {
	let matches = app().get_matches();

	let config_file =
		Path::new(constants::CONFIG_DIR).join("zeus.toml");
	debug!("Config file: {:?}", config_file);

	let file_data = std::fs::read_to_string(&config_file)
		.context("Unable to read config file")?;
	drop(config_file);

	let opts = GlobalOptions::new(&file_data, &matches)
		.context("Unable to parse config file")?;
	trace!("global opts = {:#?}", &opts);

	let global_opts = opts;
	init_global(&global_opts)?;

	trace!("Operation: {:?}", matches.subcommand_name());

	let mut aur = aur::Aur::new(&global_opts.aur_url)
		.context("Unable to initialize AUR client")?;

	let mut db =
		db::Db::new(&global_opts.build_dir).context(format!(
			"Unable to initialize database {}",
			&global_opts.build_dir.display()
		))?;

	let get_lock =
		|| db.lock().context("Unable to obtain lock on database");

	let load_runtime = |name: &str| {
		env::set_current_dir(Path::new(constants::DATA_DIR))
			.context(format!(
				"Unable to move into {}",
				constants::DATA_DIR
			))?;

		Runtime::load(
			Path::new(constants::LIB_DIR)
				.join("runtimes")
				.join(format!("librt_{name}.so")),
		)
		.context(format!("Unable to load runtime {name}"))
	};

	// These `unwrap`s should be safe as the call to `GlobalOpts::new()` should
	// have already validated that the data is a valid schema.

	match matches.subcommand() {
		Some(("sync", m)) => {
			let opts =
				SyncOptions::new(&file_data, m)?;
			trace!("sync opts = {:#?}", &opts);

			let mut runtime = load_runtime(&global_opts.runtime)?;
			runtime.init(&global_opts)?;

			sync::sync(&mut runtime, get_lock()?, &mut aur, global_opts, opts)
		},
		Some(("remove", m)) => {
			let opts =
				RemoveOptions::new(&file_data, m)?;
			trace!("remove opts = {:#?}", &opts);

			let mut runtime = load_runtime(&global_opts.runtime)?;
			runtime.init(&global_opts)?;

			remove::remove(&mut runtime, get_lock()?, global_opts, opts)
		},
		Some(("build", m)) => {
			let opts =
				BuildOptions::new(&file_data, m)?;
			trace!("build opts = {:#?}", &opts);

			get_lock()?;

			let mut runtime = load_runtime(&global_opts.runtime)?;
			runtime.init(&global_opts)?;

			build::build(&mut runtime, global_opts)
		},
		Some(("query", m)) => {
			let opts =
				QueryOptions::new(&file_data, m)?;
			trace!("query opts = {:#?}", &opts);

			query::query(&mut db, &mut aur, opts)
		},
		Some(("completions", m)) => {
			let opts = CompletionOptions::new(&file_data, m)?;
			trace!("completions opts = {:#?}", &opts);

			completions::completions(opts)
		},
		Some(("runtime", m)) => {
			let opts =
				RuntimeOptions::new(&file_data, m)?;
			trace!("runtime opts = {:#?}", &opts);

			runtime::runtime(opts)
		},
		_ => panic!("How did we get here? This is a bug. Please run with `--level trace` and report this."),
	}
}

/// Initialize the environment
fn init_global(opts: &GlobalOptions) -> Result<()> {
	match opts.color {
		Color::Always => log::set_color_enabled(true),
		Color::Never => log::set_color_enabled(false),
		_ => {},
	};

	set_log_level!(opts.log_level.clone());

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

pub(self) fn start_builder(
	runtime: &mut Runtime,
	gopts: GlobalOptions,
	operation: Message,
) -> Result<Response> {
	/*
	! IMPORTANT: Do not use anything that will write to stdout or stderr here,
	!            because we expect the runtime to have attached the builder to
	!            them and doing so might mess up the output.
	*/

	use ::std::thread;

	let machine_name = gopts.machine_name.clone();

	let builder = thread::Builder::new()
		.spawn(move || -> Result<Response> {
			use crate::ipc::Listener;

			let mut ipc =
				Listener::new(gopts.build_dir.join(".zeus.sock"))
					.context("Unable to create listener")?;

			ipc.send(Message::Init(gopts))
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

//mod ops;

use ::std::path::Path;

mod prelude {
	pub(crate) use crate::{
		aur,
		config::*,
		constants, db,
		error::*,
		log::{self, macros::*, Colorize},
		runtime::Runtime,
	};
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

	// These `unwrap`s should be safe as the call to `GlobalOpts::new()` should
	// have already validated that the data is a valid schema.

	let load_runtime = |name: &str| {
		Runtime::load(
			Path::new(constants::LIB_DIR)
				.join("runtimes")
				.join(format!("librt_{}.so", name)),
		)
		.context(format!("Unable to load runtime {}", name))
	};

	let mut aur =
		aur::Aur::new(&global_opts.aur_url, constants::AUR_IDENTITY)
			.context("Unable to initialize AUR client")?;

	let mut db = db::Db::new(&global_opts.build_dir);

	let mut get_lock = || {
		db.lock().context(format!(
			"Unable to obtain lock on {}",
			db.lockfile().display()
		))
	};

	match matches.subcommand() {
		Some(("sync", m)) => {
			let opts =
				SyncOptions::new(&file_data, m).unwrap();
			trace!("sync opts = {:#?}", &opts);

			get_lock()?;

			let mut runtime = load_runtime(&global_opts.runtime)?;
			runtime.init(&global_opts)?;

			sync::sync(&mut runtime, &mut db, &mut aur, global_opts, opts)
		},
		Some(("remove", m)) => {
			let opts =
				RemoveOptions::new(&file_data, m).unwrap();
			trace!("remove opts = {:#?}", &opts);

			get_lock()?;

			let mut runtime = load_runtime(&global_opts.runtime)?;
			runtime.init(&global_opts)?;

			remove::remove(&mut runtime, &mut db,global_opts, opts)
		},
		Some(("build", m)) => {
			let opts =
				BuildOptions::new(&file_data, m).unwrap();
			trace!("build opts = {:#?}", &opts);

			get_lock()?;

			let mut runtime = load_runtime(&global_opts.runtime)?;
			runtime.init(&global_opts)?;

			build::build(&mut runtime, global_opts)
		},
		Some(("query", m)) => {
			let opts =
				QueryOptions::new(&file_data, m).unwrap();
			trace!("query opts = {:#?}", &opts);

			query::query(&mut db, &mut aur, opts)
		},
		Some(("completions", m)) => {
			let opts = CompletionOptions::new(&file_data, m)
				.unwrap();
			trace!("completions opts = {:#?}", &opts);

			completions::completions(opts)
		},
		Some(("runtime", m)) => {
			let opts =
				RuntimeOptions::new(&file_data, m).unwrap();
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

	Ok(())
}

pub(self) fn start_builder(
	_runtime: &mut Runtime,
) -> Result<Vec<String>> {
	/*
	! IMPORTANT: Do not use anything that will write to stdout or stderr here,
	!            because we expect the runtime to have attached the builder to
	!            them and doing so might mess up the output.
	*/

	/*
	use std::thread::Builder;

	let machine_name = config.global_opts.machine_name.clone();

	let builder =
		Builder::new()
			.spawn(move || -> Result<Vec<String>> {
				use crate::ipc::{Listener, Message};

				let mut ipc = Listener::new(
					config.global_opts.build_dir.join(".zeus.sock"),
				)
				.context("failed to initialize listener")?;

				ipc.send(Message::BuilderInit { config })
					.context("failed to initialize builder")?;

				match ipc
					.recv()
					.context("failed to read builder response")?
				{
					Message::Response { packages } => Ok(packages),
					r => {
						debug!(
							"Unexpected response from builder: {:?}",
							r
						);

						Err(Error::new("received unexpected response from builder"))
					},
				}
			})
			.context("failed to spawn builder thread")?;

	runtime
		.start_machine(&machine_name)
		.context("failed to start builder")?;

	debug!("Waiting for builder thread to finish...");
	builder.join().unwrap()
	*/

	todo!("finish start_builder")
}

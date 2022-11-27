mod aur;
mod cli;
mod config;
mod constants;
mod db;
mod error;
//mod ipc;
mod log;
mod runtime;

fn main() {
	// 	use crate::config::{Color, Config};
	//
	// 	let args = Config::parse();
	//
	// 	info!("args = {:#?}", &args);
	//
	// 	set_max_log_level!(args.log_level.clone());

	debug!("Version: {}", constants::VERSION);

	if let Err(e) = cli::init() {
		fatal!("{}", e);
	}

	//matches.get_one(id)

	//let command = command.subcommand(Commang::new("sync"));

	/*
	{
		match args.color {
			Color::Always => log::set_color_enabled(true),
			Color::Never => log::set_color_enabled(false),
			Color::Auto => {},
		}

		inquire::set_global_render_config(
			inquire::ui::RenderConfig::default_colored()
				.with_prompt_prefix(
					inquire::ui::Styled::new("=>")
						.with_fg(inquire::ui::Color::LightGreen),
				)
				.with_unselected_checkbox(inquire::ui::Styled::new(
					" ",
				))
				.with_selected_checkbox(
					inquire::ui::Styled::new("*")
						.with_fg(inquire::ui::Color::LightGreen),
				),
		);
	}

	{
		// needed for the package database
		use nix::sys::stat::{umask, Mode};

		umask(Mode::S_IRWXO); // umask 007
	}

	trace!("operation = {:?}", &args.operation);

	let res = cli::ops::run_operation(args);

	match res {
		Ok(_) => exit(0),
		Err(e) => {
			error!("{}", e);
			exit(1);
		},
	}
	*/
}

mod aur;
mod config;
mod constants;
mod error;
mod ipc;
mod log;
mod machine;
mod ops;
mod package;

use std::process::exit;

use clap::Parser;

fn main() {
	let args = config::Config::parse();

	unsafe { log::LOGGER.level = args.global_opts.log_level.clone() }

	{
		use config::Color;
		match args.global_opts.color {
			Color::Always => colored::control::set_override(true),
			Color::Never => {
				colored::control::set_override(false);
			},
			_ => {},
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

	let res = ops::run_operation(args);

	match res {
		Ok(_) => exit(0),
		Err(e) => {
			error!("{}", e);
			exit(1);
		},
	}
}

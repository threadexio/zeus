mod aur;
mod cli;
mod config;
mod error;
mod log;
mod ops;
mod util;

use aur::Aur;

use std::env::remove_var;
use std::process::exit;

#[tokio::main]
async fn main() {
	remove_var("DOCKER_HOST"); // just to make sure

	let args = cli::build().get_matches();

	let mut logger = log::Logger {
		debug: args.is_present("debug"),
		out: log::Stream::Stdout,
		..Default::default()
	};

	match args.value_of("color") {
		Some("always") => {
			log::control::set_override(true);
		},
		Some("never") => {
			log::control::set_override(false);
		},
		_ => {},
	}

	let mut cfg = config::AppConfig {
		debug: logger.debug,
		force: args.is_present("force"),

		// this should never fail, we set the default value in cli.rs
		builddir: args.value_of("builddir").unwrap().to_owned(),

		aur: Aur::new()
			.host(args.value_of("aur").unwrap().to_owned())
			.build(),

		// initialization of the rest will be in the code that handles the subcommands
		..Default::default()
	};

	if cfg.force {
		cfg.buildargs.push("-f".to_owned());
	}

	let (command_name, command_args) = args.subcommand().unwrap();

	let res = ops::run_operation(
		command_name,
		&mut logger,
		&mut cfg,
		command_args,
	)
	.await;

	match res {
		Ok(_) => exit(0),
		Err(e) => {
			log_error!(logger, e.caller, "{}", e.message);
			exit(1);
		},
	}
}

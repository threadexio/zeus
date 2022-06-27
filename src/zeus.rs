mod cli;
mod ops;
mod term;
mod util;

pub mod aur;
pub mod config;
pub mod error;
pub mod log;

use std::env;
use std::process::exit;

#[tokio::main]
async fn main() {
	crate::env::remove_var("DOCKER_HOST"); // just to make sure

	let args = cli::build().get_matches();

	let mut term = term::Terminal::new(log::Logger {
		debug: args.is_present("debug"),
		..Default::default()
	});

	match args.value_of("color") {
		Some("always") => {
			term.color(true);
		},
		Some("never") => {
			term.color(false);
		},
		_ => {},
	}

	let mut cfg = config::AppConfig {
		debug: term.log.debug,
		force: args.is_present("force"),

		// this should never fail, we set the default value in cli.rs
		builddir: args.value_of("builddir").unwrap().to_owned(),

		aur: aur::Aur::new()
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
		&mut term,
		&mut cfg,
		command_args,
	)
	.await;

	match res {
		Ok(_) => exit(0),
		Err(e) => {
			error!(term.log, &e.caller, "{}", e.message);
			exit(1);
		},
	}
}

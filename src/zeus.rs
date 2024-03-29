mod cli;
mod lock;
mod message;
mod ops;
mod term;
mod unix;

pub mod aur;
pub mod config;
pub mod error;
pub mod log;

pub mod machine;

use std::process::exit;

fn main() {
	let args = cli::build().get_matches();

	let mut term = term::Terminal::new();

	unsafe {
		log::LOGGER.debug = args.is_present("debug");
	}

	match args.value_of("color") {
		Some("always") => {
			term.color(true);
		},
		Some("never") => {
			term.color(false);
		},
		_ => {},
	}

	let (command_name, command_args) = args.subcommand().unwrap();

	let mut cfg = config::AppConfig {
		operation: config::Operation::from(command_name),

		debug: args.is_present("debug"),
		force: args.is_present("force"),

		// this should never fail, we set the default value in cli.rs
		build_dir: args.value_of("builddir").unwrap().to_owned(),

		aur: aur::Aur::new()
			.host(args.value_of("aur").unwrap().to_owned())
			.build(),

		runtime: args.value_of("rt").unwrap().to_owned(),
		runtime_dir: args.value_of("rtdir").unwrap().to_owned(),

		// initialization of the rest will be in the code that handles the subcommands
		..Default::default()
	};

	if cfg.force {
		cfg.build_args.push("-f".to_owned());
	}

	let res = ops::run_operation(&mut term, cfg, command_args);

	match res {
		Ok(_) => exit(0),
		Err(e) => {
			error!(&e.caller, "{}", e.message);
			exit(1);
		},
	}
}

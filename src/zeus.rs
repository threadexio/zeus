#![allow(dead_code)]
mod aur;
mod cache;
mod config;
mod error;
mod log;
mod message;
mod ops;
mod term;
mod unix;

pub mod machine;

use std::process::exit;

use clap::Parser;

fn main() {
	let args = config::Config::parse();

	let mut term = term::Terminal::new();

	unsafe { log::LOGGER.level = args.log_level.clone() }

	{
		use config::Color;
		match args.color {
			Color::Always => {
				term.color(true);
			},
			Color::Never => {
				term.color(false);
			},
			_ => {},
		}
	}

	let res = ops::run_operation(&mut term, args);

	match res {
		Ok(_) => exit(0),
		Err(e) => {
			error!("{}", e);
			exit(1);
		},
	}
}

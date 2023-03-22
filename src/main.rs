#![deny(clippy::correctness)]
#![warn(
	clippy::all,
	clippy::style,
	clippy::complexity,
	clippy::perf,
	clippy::unwrap_used
)]

mod aur;
mod cli;
mod config;
mod constants;
mod db;
mod ipc;
mod runtime;
mod term;

fn main() {
	{
		use nix::sys::stat::{umask, Mode};
		umask(Mode::S_IRWXO);
	}

	let mut term = term::Terminal::new();

	if let Err(e) = cli::init(&mut term) {
		term.fatal(format!("{:#}", e));
	}
}

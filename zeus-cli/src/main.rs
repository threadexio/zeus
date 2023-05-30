#![deny(clippy::correctness)]
#![warn(
	clippy::all,
	clippy::style,
	clippy::complexity,
	clippy::perf,
	clippy::unwrap_used
)]

use zeus_term::Terminal;

mod cli;

fn main() {
	{
		use nix::sys::stat::{umask, Mode};
		umask(Mode::S_IRWXO);
	}

	let mut term = Terminal::new();

	if let Err(e) = cli::init(&mut term) {
		term.fatal(format!("{:#}", e));
	}
}

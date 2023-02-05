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
mod log;
mod runtime;

fn main() {
	if let Err(e) = cli::init() {
		fatal!("{:#}", e);
	}
}

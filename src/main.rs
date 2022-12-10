#![deny(clippy::all)]

mod aur;
mod cli;
mod config;
mod constants;
mod db;
mod error;
mod ipc;
mod log;
mod runtime;

fn main() {
	if let Err(e) = cli::init() {
		fatal!("{}", e);
	}
}

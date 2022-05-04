mod aur;
mod config;
mod error;
mod log;

use log::Level;

use std::env;
use std::io::Read;
use std::os::unix::net::UnixStream;
use std::path;
use std::process::exit;
use std::process::Command;

fn build_packages(cfg: config::AppConfig) {
	//! TODO: Error handling please

	for package in cfg.packages {
		let pkg_dir = path::Path::new(&package);

		if !pkg_dir.exists() {
			let _ = Command::new("/usr/bin/git")
				.arg("clone")
				.arg(format!("{}/{}.git", cfg.aur.get_url(), &package))
				.status();
		}

		let _ = env::set_current_dir(pkg_dir);

		if cfg.upgrade {
			let _ = Command::new("/usr/bin/git")
				.arg("pull")
				.arg("origin")
				.arg("master")
				.status();
		}

		let _ = Command::new("/usr/bin/makepkg")
			.arg("-s")
			.arg("--needed")
			.arg("--noconfirm")
			.arg("--noprogressbar")
			.args(&cfg.buildargs)
			.status();

		let _ = env::set_current_dir("/build/");
	}
}

fn main() {
	let mut logger = log::Logger::new(log::Stream::Stdout, log::ColorChoice::Auto);

	logger.v(
		Level::Info,
		"builder",
		format!("Version: {}", config::PROGRAM_VERSION),
	);

	let socket_path = format!("{}.sock", config::PROGRAM_NAME);
	let mut stream = match UnixStream::connect(&socket_path) {
		Ok(v) => v,
		Err(e) => {
			logger.v(
				Level::Error,
				"builder",
				format!("Cannot connect to socket: {}", e),
			);
			exit(1);
		}
	};

	let mut data = vec![0u8; 1024 * 8];
	let data_len: usize;
	match stream.read(&mut data[..]) {
		Ok(v) => {
			data_len = v;
		}
		Err(e) => {
			logger.v(
				Level::Error,
				"builder",
				format!("Cannot read data from socket: {}", e),
			);
			exit(1);
		}
	}

	// the &data[..data_len] is needed because serde_json doesn't stop parsing on a null byte
	let cfg: config::AppConfig = match serde_json::from_slice(&data[..data_len]) {
		Ok(v) => v,
		Err(e) => {
			logger.v(
				Level::Error,
				"builder",
				format!("Cannot deserialize config: {}", e),
			);
			exit(1);
		}
	};

	build_packages(cfg);
}

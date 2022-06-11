mod aur;
mod config;
mod error;
mod log;

use error::{zerr, Result, ZeusError};

use std::env;
use std::io::Read;
use std::os::unix::net::UnixStream;
use std::path;
use std::process::exit;
use std::process::Command;

fn build_packages(cfg: &config::AppConfig) -> Result<Vec<&str>> {
	let mut new_packages: Vec<&str> = vec![];

	for package in &cfg.packages {
		zerr!(
			env::set_current_dir("/build"),
			"fs".to_owned(),
			&format!("Cannot change cwd to /build",)
		);

		let pkg_dir = path::Path::new(&package);

		if !pkg_dir.exists() {
			let _ = Command::new("/usr/bin/git")
				.arg("clone")
				.arg(format!(
					"{}/{}.git",
					cfg.aur.get_url(),
					&package
				))
				.status();
		}

		zerr!(
			env::set_current_dir(pkg_dir),
			"fs".to_owned(),
			&format!(
				"Cannot change directory to {}",
				pkg_dir.display()
			)
		);

		if cfg.upgrade {
			let status = zerr!(
				Command::new("/usr/bin/git")
					.arg("pull")
					.arg("origin")
					.arg("master")
					.status(),
				"cmd".to_owned(),
				"Cannot start git"
			);

			if !status.success() {
				return Err(ZeusError::new(
					"cmd".to_owned(),
					format!(
						"git exited with: {}",
						status.code().unwrap_or(-99999)
					),
				));
			}
		}

		let status = zerr!(
			Command::new("/usr/bin/makepkg")
				.arg("-s")
				.arg("--needed")
				.arg("--noconfirm")
				.arg("--noprogressbar")
				.args(&cfg.buildargs)
				.status(),
			"cmd".to_owned(),
			"Cannot start makepkg"
		);

		if !status.success() {
			continue;
		}

		new_packages.push(package);
	}

	Ok(new_packages)
}

fn main() {
	let logger = log::Logger {
		out: log::Stream::Stdout,
		..Default::default()
	};

	logger.i(
		"builder",
		format!("Version: {}", config::PROGRAM_VERSION),
	);

	let socket_path = format!("{}.sock", config::PROGRAM_NAME);
	let mut stream = match UnixStream::connect(&socket_path) {
		Ok(v) => v,
		Err(e) => {
			logger.e(
				"unix",
				format!("Cannot connect to socket: {}", e),
			);
			exit(1);
		},
	};

	let mut data = vec![0u8; 1024 * 8];
	let data_len: usize;
	match stream.read(&mut data[..]) {
		Ok(v) => {
			data_len = v;
		},
		Err(e) => {
			logger.e(
				"unix",
				format!("Cannot read data from socket: {}", e),
			);
			exit(1);
		},
	}

	// the &data[..data_len] is needed because serde_json doesn't stop parsing on a null byte
	let cfg: config::AppConfig =
		match serde_json::from_slice(&data[..data_len]) {
			Ok(v) => v,
			Err(e) => {
				logger.e(
					"zeus",
					format!("Cannot deserialize config: {}", e),
				);
				exit(1);
			},
		};

	let pkgs = match build_packages(&cfg) {
		Ok(v) => v,
		Err(e) => {
			logger.e(e.caller, e.message);
			exit(1);
		},
	};

	if cfg.upgrade {
		logger.i("builder", "Upgraded packages:");
	} else {
		logger.i("builder", "Built packages:");
	}

	println!("{}", pkgs.join("\n"));
}

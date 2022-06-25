use std::env;
use std::fs;
use std::io::Read;
use std::os::unix::net::UnixStream;
use std::path;
use std::process::exit;
use std::process::Command;

mod aur;
mod config;
mod error;
mod log;

use config::Operation;
use error::{zerr, Result, ZeusError};
use log::Colorize;

fn build_packages<'a>(
	cfg: &'a config::AppConfig,
) -> Result<Vec<&'a str>> {
	let mut new_packages: Vec<&str> = Vec::new();

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

fn remove_packages<'a>(
	logger: &log::Logger,
	cfg: &'a config::AppConfig,
) -> Result<Vec<&'a str>> {
	let mut removed_packages: Vec<&str> = Vec::new();

	for package in &cfg.packages {
		let pkg_path = path::Path::new(&package);

		if pkg_path.exists() && pkg_path.is_dir() {
			match fs::remove_dir_all(pkg_path) {
				Ok(_) => {
					removed_packages.push(package);
				},
				Err(e) => {
					log_warn!(
						logger,
						"fs",
						"Cannot remove package directory {}: {}",
						pkg_path.display(),
						e
					);
				},
			}
		} else {
			log_warn!(logger, "zeus", "Package has not been synced");
		}
	}

	Ok(removed_packages)
}

fn main() {
	let mut logger = log::Logger::default();

	log_info!(
		logger,
		"builder",
		"Version: {}",
		config::PROGRAM_VERSION.bright_blue()
	);

	match env::set_current_dir("/build") {
		Ok(_) => {},
		Err(e) => {
			log_error!(
				logger,
				"builder",
				"Cannot change directory to {}: {}",
				"/build",
				e
			);
			exit(1);
		},
	}

	let socket_path = format!("{}.sock", config::PROGRAM_NAME);
	let mut stream = match UnixStream::connect(&socket_path) {
		Ok(v) => v,
		Err(e) => {
			log_error!(
				logger,
				"unix",
				"Cannot connect to socket {}: {}",
				socket_path,
				e
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
			log_error!(
				logger,
				"unix",
				"Cannot read data from socket: {}",
				e
			);
			exit(1);
		},
	}

	let cfg: config::AppConfig =
		match serde_json::from_slice(&data[..data_len]) {
			Ok(v) => v,
			Err(e) => {
				log_error!(
					logger,
					"zeus",
					"Cannot deserialize config: {}",
					e
				);
				exit(1);
			},
		};

	logger.debug = cfg.debug;

	match cfg.operation {
		Operation::Sync => match build_packages(&cfg) {
			Err(e) => {
				log_error!(logger, e.caller, "{}", e.message);
			},
			Ok(pkgs) => {
				if cfg.upgrade {
					println!("Upgraded packages:");
				} else {
					println!("Built packages:");
				}

				for pkg in pkgs {
					println!(
						"{} {}",
						"=>".green(),
						pkg.bright_white().bold()
					)
				}
			},
		},
		Operation::Remove => match remove_packages(&logger, &cfg) {
			Err(e) => {
				log_error!(logger, e.caller, "{}", e.message);
			},
			Ok(pkgs) => {
				println!("Removed packages:");

				for pkg in pkgs {
					println!(
						"{} {}",
						"=>".green(),
						pkg.bright_white().bold()
					)
				}
			},
		},
		_ => {
			log_debug!(
				logger,
				"builder",
				"operation = {:?}",
				cfg.operation
			);
		},
	}
}

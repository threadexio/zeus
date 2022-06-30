use std::env;
use std::fs;
use std::path;
use std::process::exit;
use std::process::Command;

mod aur;
mod config;
mod error;
mod log;
mod machine;
mod message;
mod unix;

use colored::Colorize;
use config::Operation;
use error::{Result, ZeusError};
use message::Message;

fn build_packages<'a>(
	cfg: &'a config::AppConfig,
) -> Result<Vec<&'a str>> {
	let mut new_packages: Vec<&str> = Vec::new();

	for package in &cfg.packages {
		zerr!(
			env::set_current_dir("/build"),
			"fs".to_owned(),
			"Cannot change cwd to /build"
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
			"Cannot change directory to {}",
			pkg_dir.display()
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
				.args(&cfg.build_args)
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
					warn!(
						logger,
						"fs",
						"Cannot remove package directory {}: {}",
						pkg_path.display(),
						e
					);
				},
			}
		} else {
			warn!(logger, "zeus", "Package has not been synced");
		}
	}

	Ok(removed_packages)
}

fn main() {
	let mut logger = log::Logger::default();

	info!(
		logger,
		"builder",
		"Version: {}",
		config::PROGRAM_VERSION.bright_blue()
	);

	match env::set_current_dir("/build") {
		Ok(_) => {},
		Err(e) => {
			error!(
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
	let mut channel = match unix::connect::<Message, _>(socket_path) {
		Ok(v) => v,
		Err(e) => {
			error!(logger, "unix", "Cannot connect to host: {}", e);
			exit(1);
		},
	};

	let cfg: config::AppConfig = match channel.recv() {
		Ok(v) => match v {
			Message::Config(c) => c,
			m => {
				error!(
					logger,
					"builder", "Expected config, got {:?}", m
				);
				exit(1);
			},
		},
		Err(e) => {
			error!(
				logger,
				"builder", "Cannot deserialize config: {}", e
			);
			exit(1);
		},
	};

	logger.debug = cfg.debug;

	match cfg.operation {
		Operation::Sync => match build_packages(&cfg) {
			Err(e) => {
				error!(logger, &e.caller, "{}", e.message);
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
				error!(logger, &e.caller, "{}", e.message);
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
			debug!(
				logger,
				"builder", "operation = {:?}", cfg.operation
			);
		},
	};

	channel.send(Message::Done).unwrap();
}

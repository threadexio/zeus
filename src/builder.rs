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
					warning!(
						"fs",
						"Cannot remove package directory {}: {}",
						pkg_path.display(),
						e
					);
				},
			}
		} else {
			warning!("zeus", "Package has not been synced");
		}
	}

	Ok(removed_packages)
}

fn main() {
	info!("builder", "Version: {}", config::VERSION.bright_blue());

	match env::set_current_dir("/build") {
		Ok(_) => {},
		Err(e) => {
			error!(
				"builder",
				"Cannot change directory to {}: {}", "/build", e
			);
			exit(1);
		},
	}

	let socket_path = format!("{}.sock", config::NAME);
	let mut channel = match unix::connect::<Message, _>(socket_path) {
		Ok(v) => v,
		Err(e) => {
			error!("unix", "Cannot connect to host: {}", e);
			exit(1);
		},
	};

	let cfg: config::AppConfig = match channel.recv() {
		Ok(v) => match v {
			Message::Config(c) => c,
			m => {
				error!("builder", "Expected config, got {:?}", m);
				exit(1);
			},
		},
		Err(e) => {
			error!("builder", "Cannot deserialize config: {}", e);
			exit(1);
		},
	};

	unsafe {
		log::LOGGER.debug = cfg.debug;
	}

	let op_res = match cfg.operation {
		Operation::Sync => build_packages(&cfg),
		Operation::Remove => remove_packages(&cfg),
		_ => {
			error!(
				"builder",
				"Unexpected operation: {:?}", cfg.operation
			);
			exit(1);
		},
	};

	match op_res {
		Ok(v) => channel.send(Message::Success(
			v.iter().map(|x| -> String { x.to_string() }).collect(),
		)),
		Err(e) => channel.send(Message::Failure(e.to_string())),
	}
	.unwrap();
}

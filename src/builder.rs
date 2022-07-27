use std::process::{exit, ExitStatus};

use std::os::unix::net::UnixStream;

mod aur;
mod config;
mod error;
mod log;
mod machine;
mod message;
mod unix;

use colored::Colorize;
use config::{AppConfig, Operation};
use error::{Result, ZeusError};
use message::Message;

fn chdir(dir: &str) -> Result<()> {
	zerr!(
		std::env::set_current_dir(dir),
		"fs",
		"Cannot change cwd to \"{}\"",
		dir
	);

	Ok(())
}

fn run_command(arg0: &str, args: &[&str]) -> Result<ExitStatus> {
	use std::process::Command;

	info!("builder", "Running: {} {}", arg0, args.join(" "));

	let status = zerr!(
		Command::new(arg0).args(args).status(),
		"builder",
		"Cannot start: {}",
		arg0
	);

	Ok(status)
}

fn update_package() -> Result<()> {
	let r = run_command("git", &["pull", "-f"])?;

	if !r.success() {
		return Err(ZeusError::new(
			"builder".to_owned(),
			format!("git failed with: {}", r.code().unwrap_or(-1)),
		));
	}

	Ok(())
}

fn clone_package(cfg: &AppConfig, package_name: &str) -> Result<()> {
	let status = run_command(
		"git",
		&[
			"clone",
			"--",
			&format!("{}/{}.git", cfg.aur.get_url(), &package_name),
		],
	)?;

	if !status.success() {
		return Err(ZeusError::new(
			"builder".to_owned(),
			format!(
				"git failed with: {}",
				status.code().unwrap_or(-1)
			),
		));
	}

	Ok(())
}

fn make_package(cfg: &AppConfig) -> Result<()> {
	let args: Vec<&str> =
		cfg.build_args.iter().map(|x| x.as_str()).collect();

	let status = run_command(
		"makepkg",
		&[
			&[
				"--needed",
				"--noconfirm",
				"--noprogressbar",
				"-s",
				"-i",
			],
			args.as_slice(),
		]
		.concat(),
	)?;

	if !status.success() {
		return Err(ZeusError::new(
			"builder".to_owned(),
			format!(
				"makepkg failed with: {}",
				status.code().unwrap_or(-1)
			),
		));
	}

	Ok(())
}

fn build_package(cfg: &AppConfig, package_name: &str) -> Result<()> {
	use std::path::Path;
	if !Path::new(package_name).exists() {
		clone_package(&cfg, package_name)?;
	}

	chdir(package_name)?;

	if cfg.upgrade {
		update_package()?;
	}

	make_package(&cfg)?;

	Ok(())
}

fn build_packages(
	cfg: &AppConfig,
	build_root: &str,
) -> Result<Vec<String>> {
	let mut packages: Vec<String> = vec![];
	for package in &cfg.packages {
		chdir(build_root)?;

		match build_package(&cfg, package) {
			Err(e) => {
				warning!("builder", "{}", e);
				continue;
			},
			_ => {},
		}

		packages.push(package.to_owned())
	}

	Ok(packages)
}

fn remove_packages<'a>(
	cfg: &'a config::AppConfig,
) -> Result<Vec<String>> {
	let mut removed_packages: Vec<String> = vec![];

	use std::fs;
	use std::path::Path;
	for package in &cfg.packages {
		let pkg_path = Path::new(&package);

		if pkg_path.exists() && pkg_path.is_dir() {
			match fs::remove_dir_all(pkg_path) {
				Ok(_) => {
					removed_packages.push(package.to_owned());
				},
				Err(e) => {
					warning!(
						"fs",
						"Cannot remove package directory \"{}\": {}",
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

	match chdir("/build") {
		Err(e) => {
			error!(&e.caller, "{}", e.message);
			exit(1);
		},
		_ => {},
	};

	let (mut tx, mut rx) = match UnixStream::connect(".zeus.sock") {
		Ok(v) => channels::channel::<Message, _>(v),
		Err(e) => {
			error!(
				"builder",
				"Cannot connect to socket \".zeus.sock\": {}", e
			);
			exit(1);
		},
	};

	let cfg: config::AppConfig = match rx.recv() {
		Ok(v) => match v {
			Message::Config(c) => c,
			_ => {
				error!("builder", "Expected config, got: {:?}", v);
				exit(1);
			},
		},
		Err(e) => {
			error!("builder", "Cannot receive config: {}", e);
			exit(1);
		},
	};

	unsafe {
		log::LOGGER.debug = cfg.debug;
	}

	let op_res = match cfg.operation {
		Operation::Sync => build_packages(&cfg, "/build"),
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
		Ok(v) => tx.send(Message::Success(v)),
		Err(e) => tx.send(Message::Failure(e.message)),
	}
	.unwrap();
}

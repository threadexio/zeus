#![allow(dead_code)]
mod aur;
mod config;
mod error;
mod log;
mod message;
mod unix;

use config::{
	Config, GlobalOptions, Operation, RemoveOptions, SyncOptions,
};
use error::*;
use message::{Message, PackageMeta};

use colored::Colorize;

use std::process::exit;

fn sync(
	c: &mut unix::UnixChannels<Message>,
	gopts: &GlobalOptions,
	opts: SyncOptions,
) -> Result<()> {
	let (tx, _) = c;

	for package in &opts.packages {
		let package =
			match util::sync_package(gopts, &opts, &package.name) {
				Err(e) => {
					error!("{}", e);
					continue;
				},
				Ok(v) => v,
			};

		err!(
			tx.send(Message::PackageMeta(package)),
			"Cannot send build progress to host"
		);
	}

	Ok(())
}

fn remove(
	c: &mut unix::UnixChannels<Message>,
	_gopts: &GlobalOptions,
	opts: RemoveOptions,
) -> Result<()> {
	for package in opts.packages {
		err!(
			util::remove_package(&package.name),
			"Cannot remove package directory"
		);

		let _ = c.0.send(Message::PackageMeta(PackageMeta {
			package,
			files: vec![],
		}));
	}

	Ok(())
}

fn main() {
	info!("Version: {}", config::constants::VERSION.bright_blue());

	match util::chdir("/build") {
		Err(e) => {
			error!("Error changing directory to /build: {}", e);
			exit(1);
		},
		_ => {},
	};

	let mut c = match setup_connection() {
		Ok(v) => v,
		Err(e) => {
			error!("Error connecting to host socket: {}", e);
			exit(1);
		},
	};

	let Config { global_opts: gopts, operation: op } =
		match get_config(&mut c) {
			Ok(v) => v,
			Err(e) => {
				error!("Error receiving config: {}", e);
				exit(1);
			},
		};

	let res = match op {
		Operation::Sync(v) => sync(&mut c, &gopts, v),
		Operation::Remove(v) => remove(&mut c, &gopts, v),
		o => panic!("unexpected operation: {:?}", o),
	};

	let (tx, _) = &mut c;

	match res {
		Err(e) => {
			error!("{}", e);
			let _ = tx.send(Message::Failure(e.to_string()));
			exit(-1);
		},
		Ok(_) => {
			let _ = tx.send(Message::Success);
			exit(0);
		},
	}
}

fn setup_connection() -> Result<unix::UnixChannels<Message>> {
	use std::os::unix::net::UnixStream;
	Ok(channels::channel(UnixStream::connect(".zeus.sock")?))
}

fn get_config(c: &mut unix::UnixChannels<Message>) -> Result<Config> {
	let (_, rx) = c;

	match rx.recv() {
		Ok(v) => match v {
			Message::Config(cfg) => Ok(cfg),
			_ => Err(other!("received unexpected message: {:?}", v)),
		},
		Err(e) => Err(other!("failed to receive config: {}", e)),
	}
}

mod util {
	use std::path::Path;

	use std::process::ExitStatus;

	use super::*;

	pub fn chdir<D: AsRef<Path> + ?Sized>(path: &D) -> Result<()> {
		std::env::set_current_dir(path)
	}

	fn run_command(arg0: &str, args: &[&str]) -> Result<ExitStatus> {
		use std::process::Command;

		info!("Running: {} {}", arg0, args.join(" "));

		let status = err!(
			Command::new(arg0).args(args).status(),
			"Cannot start: {}",
			arg0
		);

		Ok(status)
	}

	pub fn get_package_files() -> Result<Vec<String>> {
		use std::process::Command;

		let output =
			Command::new("makepkg").arg("--packagelist").output()?;

		Ok(String::from_utf8_lossy(&output.stdout)
			.lines()
			.map(|x| x.to_owned())
			.collect())
	}

	fn update_package() -> Result<()> {
		let r = run_command("git", &["pull", "-f"])?;

		if !r.success() {
			return Err(other!(
				"git failed with: {}",
				r.code().unwrap_or(-1)
			));
		}

		Ok(())
	}

	fn clone_package(
		aur: &aur::Aur,
		package_name: &str,
	) -> Result<()> {
		let status = run_command(
			"git",
			&[
				"clone",
				"--",
				&format!("{}/{}.git", aur.get_url(), &package_name),
			],
		)?;

		if !status.success() {
			return Err(other!(
				"git failed with: {}",
				status.code().unwrap_or(-1)
			));
		}

		Ok(())
	}

	fn make_package(opts: &SyncOptions) -> Result<bool> {
		let args: Vec<&str> =
			opts.build_args.iter().map(|x| x.as_str()).collect();

		let status = run_command(
			"makepkg",
			&[
				&["--needed", "--noconfirm", "--noprogressbar", "-s"],
				args.as_slice(),
			]
			.concat(),
		)?;

		if !status.success() {
			if let Some(exit_code) = status.code() {
				// 13 means a package has already been built
				if exit_code == 13 {
					return Ok(false);
				}
			}

			return Err(other!(
				"makepkg failed with: {}",
				status.code().unwrap_or(-1)
			));
		}

		Ok(true)
	}

	pub fn sync_package(
		gopts: &GlobalOptions,
		opts: &SyncOptions,
		package_name: &str,
	) -> Result<PackageMeta> {
		let package_dir = Path::new(package_name);

		if !package_dir.exists() {
			err!(
				clone_package(&gopts.aur, package_name),
				"Cannot clone package"
			);
		}

		err!(
			chdir(package_dir),
			"Cannot move inside package directory {}",
			package_name
		);

		if opts.upgrade {
			err!(update_package(), "Cannot update package");
		}

		if !err!(make_package(opts), "Cannot build package") {
			return Err(other!("Package has already been built"));
		}

		let mut ret = PackageMeta {
			package: aur::Package {
				name: package_name.to_owned(),
				..Default::default()
			},
			files: vec![],
		};

		if opts.install {
			ret.files.append(&mut get_package_files()?);
		}

		err!(
			chdir(".."),
			"Cannot move outside of package directory {}",
			package_name
		);

		Ok(ret)
	}

	pub fn remove_package(package_name: &str) -> Result<()> {
		let package_dir = Path::new(package_name);

		std::fs::remove_dir_all(package_dir)
	}
}

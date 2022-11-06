mod aur;
mod config;
mod constants;
mod error;
mod ipc;
mod log;
mod package;

use colored::Colorize;

use crate::error::*;

use crate::config::{
	Config, GlobalOptions, RemoveOptions, SyncOptions,
};

pub mod init {
	use super::*;

	use std::path::Path;

	fn ipc() -> Result<ipc::Client> {
		use std::{thread::sleep, time::Duration};

		// IMPROVE: a humble attempt to fix the race condition between starting that builder and listening on the socket

		let mut i = 0;
		loop {
			match ipc::Client::new(
				Path::new(".zeus.sock").to_path_buf(),
			) {
				Ok(v) => return Ok(v),
				Err(e) => {
					if i == 5 {
						return Err(e.context(
							"Unable to connect to IPC channel",
						));
					}

					debug!("Unable to connect to IPC channel. Retrying...");

					sleep(Duration::from_secs(1));

					i += 1;

					continue;
				},
			};
		}
	}

	fn package<'p>() -> package::PackageStore<'p> {
		package::PackageStore::new(Path::new("./"))
	}

	pub fn all<'p>(
	) -> Result<(Config, ipc::Client, package::PackageStore<'p>)> {
		let pstore = package();

		let mut ipc = ipc().context("Unable to connect to socket")?;

		let config;

		use ipc::Message;
		match ipc.recv()? {
			Message::Init(c) => {
				config = c;
			},
			m => {
				return Err(Error::new(format!(
					"Expected init message. Got: {:?}",
					m,
				)))
			},
		}

		Ok((config, ipc, pstore))
	}
}

fn main() {
	debug!("Version: {}", constants::VERSION.bright_blue());

	let (config, mut ipc, mut pstore) = match init::all() {
		Ok(v) => v,
		Err(e) => {
			error!("Unable to initialize builder: {}", e);
			std::process::exit(1);
		},
	};

	let Config { global_opts: opts, operation: op } = config;

	use config::Operation;
	let r = match op {
		Operation::Sync(v) => sync(&mut ipc, &mut pstore, &opts, v),
		Operation::Remove(v) => {
			remove(&mut ipc, &mut pstore, &opts, v)
		},
		_ => {
			debug!("Unexpected operation: {:?}", &op);
			std::process::exit(128);
		},
	};

	match r {
		Ok(_) => {},
		Err(e) => {
			error!("{}", e);
			std::process::exit(3);
		},
	};
}

/// Build packages specified in `opts.packages`. Returns packages that were successfully built.
fn sync(
	ipc: &mut ipc::Client,
	pstore: &'_ mut package::PackageStore,
	gopts: &GlobalOptions,
	mut opts: SyncOptions,
) -> Result<()> {
	opts.packages.retain(|pkg_name| {
		let mut pkg = match pstore.get_pkg(pkg_name) {
			Some(mut pkg) => {
				if opts.upgrade {
					match pkg.update().context(format!(
						"Unable to update package {}",
						pkg.name()
					)) {
						Ok(_) => {},
						Err(e) => {
							error!("{}", e);
							return false;
						},
					}
				}

				pkg
			},
			None => match pstore
				.add_pkg(
					pkg_name,
					&format!(
						"{}/{}.git",
						gopts.aur.get_url(),
						pkg_name
					),
				)
				.context(format!(
					"Unable to clone package {}",
					pkg_name
				)) {
				Ok(v) => v,
				Err(e) => {
					error!("Unable to get package: {}", e);
					return false;
				},
			},
		};

		match pkg
			.build(
				&opts
					.build_args
					.iter()
					.map(|x| x.as_str())
					.collect::<Vec<_>>(),
			)
			.context(format!(
				"Unable to build package {}",
				pkg.name()
			)) {
			Ok(_) => true,
			Err(e) => {
				error!("{}", e);
				false
			},
		}
	});

	ipc.send(ipc::Message::End(opts.packages))
		.context("Unable to send results back to zeus")?;

	Ok(())
}

/// Remove packages specified in `opts.packages`. Returns packages that were successfully removed.
fn remove(
	ipc: &mut ipc::Client,
	pstore: &mut package::PackageStore,
	_gopts: &GlobalOptions,
	mut opts: RemoveOptions,
) -> Result<()> {
	opts.packages.retain(|pkg_name| {
		let pkg = match pstore.get_pkg(pkg_name) {
			Some(v) => v,
			None => {
				error!("Unable to find package {}", pkg_name);
				return false;
			},
		};

		match pstore
			.remove_pkg(pkg)
			.context(format!("Unable to remove package {}", pkg_name))
		{
			Ok(_) => {},
			Err(e) => {
				error!("{}", e);
				return false;
			},
		}

		true
	});

	ipc.send(ipc::Message::End(opts.packages))
		.context("Unable to send results back to zeus")?;

	Ok(())
}

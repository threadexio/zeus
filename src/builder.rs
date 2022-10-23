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

	fn package() -> Result<package::PackageStore> {
		Ok(package::PackageStore::new(Path::new("./"))?)
	}

	pub fn all(
	) -> Result<(Config, ipc::Client, package::PackageStore)> {
		let pstore =
			package().context("Unable to create package store")?;

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

// TODO: Finish `sync()`
fn sync(
	_ipc: &mut ipc::Client,
	_pstore: &mut package::PackageStore,
	_gopts: &GlobalOptions,
	_opts: SyncOptions,
) -> Result<()> {
	todo!()
}

/// Remove packages specified in `opts.packages`. Returns packages that were successfully removed.
fn remove(
	ipc: &mut ipc::Client,
	pstore: &mut package::PackageStore,
	_gopts: &GlobalOptions,
	mut opts: RemoveOptions,
) -> Result<()> {
	opts.packages.retain(|x| match pstore.package(&x.name) {
		Some(v) => {
			match pstore.remove_package(v).context(format!(
				"Unable to remove package {}",
				&x.name
			)) {
				Ok(_) => true,
				Err(e) => {
					warn!("{}", e);
					false
				},
			}
		},
		None => {
			warn!("Package {} is not synced", &x.name);
			false
		},
	});

	ipc.send(ipc::Message::End(opts.packages))
		.context("Unable to send results back to zeus")?;

	Ok(())
}

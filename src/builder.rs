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
		ipc::Client::new(Path::new(".zeus.sock").to_path_buf())
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
	info!("Version: {}", constants::VERSION.bright_blue());

	let (config, mut ipc, mut pstore) = match init::all() {
		Ok(v) => v,
		Err(e) => {
			error!("Unable to initialize builder: {}", e);
			std::process::exit(1);
		},
	};

	info!("{:#?}", config);

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
		Ok(v) => v,
		Err(e) => {
			error!("{}", e);
			std::process::exit(3);
		},
	}
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

// TODO: Finish `remove()`
fn remove(
	_ipc: &mut ipc::Client,
	_pstore: &mut package::PackageStore,
	_gopts: &GlobalOptions,
	_opts: RemoveOptions,
) -> Result<()> {
	todo!()
}

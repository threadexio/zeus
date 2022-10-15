mod aur;
mod config;
mod constants;
mod error;
mod ipc;
mod log;

use colored::Colorize;

use crate::error::*;

use crate::config::{
	Config, GlobalOptions, RemoveOptions, SyncOptions,
};

pub mod init {
	use super::*;

	fn ipc() -> Result<ipc::Client> {
		use std::path::Path;

		ipc::Client::new(Path::new(".zeus.sock").to_path_buf())
	}

	pub fn all() -> Result<(Config, ipc::Client)> {
		let mut ipc = ipc()?;

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

		Ok((config, ipc))
	}
}

fn main() {
	info!("Version: {}", constants::VERSION.bright_blue());

	let (config, mut ipc) = match init::all() {
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
		Operation::Sync(v) => sync(&mut ipc, &opts, v),
		Operation::Remove(v) => remove(&mut ipc, &opts, v),
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
	_gopts: &GlobalOptions,
	_opts: SyncOptions,
) -> Result<()> {
	todo!()
}

// TODO: Finish `remove()`
fn remove(
	_ipc: &mut ipc::Client,
	_gopts: &GlobalOptions,
	_opts: RemoveOptions,
) -> Result<()> {
	todo!()
}

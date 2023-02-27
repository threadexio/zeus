#![deny(clippy::correctness)]
#![warn(
	clippy::all,
	clippy::style,
	clippy::complexity,
	clippy::perf,
	clippy::unwrap_used
)]

mod aur;
mod config;
mod constants;
mod db;
mod ipc;
mod term;

use term::Terminal;

use anyhow::{anyhow, Context, Result};

fn main() {
	let mut term = Terminal::new();

	if let Err(e) = init(&mut term) {
		term.fatal(format!("{:#}", e));
	}
}

fn init(term: &mut Terminal) -> Result<()> {
	use ipc::Message;

	let mut ipc = ipc::Client::new(".zeus.sock")
		.context("Unable to connect to zeus")?;

	let global_config;
	if let Message::Init(config) = ipc
		.recv()
		.context("Unable to receive initialization packet")?
	{
		use config::types::Color;
		match config.color {
			Color::Always => term.set_color_enabled(true),
			Color::Never => term.set_color_enabled(false),
			_ => {},
		}

		term.set_log_level(config.log_level);

		global_config = config;
	} else {
		return Err(anyhow!(
			"This is a bug! Received unexpected packet",
		));
	}

	let db = db::Db::new(
		std::env::current_dir()
			.context("Unable to get current directory")?,
	)
	.context("Unable to initialize database")?;

	let db = unsafe { db.unlocked_guard() };

	let r = match ipc
		.recv()
		.context("Unable to receive operation packet")?
	{
		Message::Sync(config) => {
			sync::sync(global_config, config, term, db)
		},
		Message::Remove(config) => {
			remove::remove(global_config, config, term, db)
		},
		p => Err(anyhow!(
			"This is a bug! Received unexpected packet: {p:#?}"
		)),
	}?;

	ipc.send(Message::Response(r))
		.context("Unable to send response")?;

	Ok(())
}

use config::GlobalConfig;
use db::Transaction;
use ipc::Response;

mod sync {
	use super::*;
	use config::SyncConfig;

	pub fn sync(
		global_config: GlobalConfig,
		config: SyncConfig,
		term: &mut Terminal,
		mut db: db::DbGuard,
	) -> Result<Response> {
		let mut res = Response::default();

		for pkg_name in &config.packages {
			match sync_package(
				&global_config,
				&config,
				term,
				&mut db,
				pkg_name,
				&mut res,
			) {
				Ok(_) => {},
				Err(e) => {
					term.error(format!("{:#}", e));
					continue;
				},
			}
		}

		Ok(res)
	}

	fn sync_package(
		global_config: &GlobalConfig,
		config: &SyncConfig,
		term: &mut Terminal,
		db: &mut db::DbGuard,
		name: &str,
		res: &mut Response,
	) -> Result<()> {
		let trans = Transaction::new()
			.clone_pkg(
				name,
				format!("{}/{}.git", global_config.aur_url, name),
				config.upgrade,
			)
			.build_pkg(name, config.build_args.iter());

		db.commit(trans)
			.context(format!("Unable to sync package {name}"))?;

		res.packages.push(name.to_string());

		match db.pkg(name) {
			Ok(pkg) => {
				res.files.extend(
					pkg.files()
						.context(format!(
							"Unable to get package files for {name}"
						))?
						.drain(..)
						.filter_map(|x| {
							x.strip_prefix(db.root())
								.map(|x| x.to_path_buf())
								.ok()
						}),
				);
			},
			_ => {
				term.warn(format!(
					"Package {name} synced but is not found in database"
				));
			},
		}

		Ok(())
	}
}

mod remove {
	use super::*;
	use config::RemoveConfig;

	pub fn remove(
		_: GlobalConfig,
		config: RemoveConfig,
		_: &mut Terminal,
		mut db: db::DbGuard,
	) -> Result<Response> {
		let mut transaction = Transaction::new();

		for pkg in &config.packages {
			transaction = transaction.remove_pkg(pkg);
		}

		db.commit(transaction)
			.context("Unable to remove all packages")?;

		let res = Response {
			packages: config.packages,
			..Default::default()
		};

		Ok(res)
	}
}

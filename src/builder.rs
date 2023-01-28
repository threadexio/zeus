#![deny(clippy::correctness)]
#![warn(
	clippy::style,
	clippy::complexity,
	clippy::perf,
	clippy::unwrap_used,
	clippy::expect_used
)]

mod aur;
mod config;
mod constants;
mod db;
mod ipc;
mod log;

use anyhow::{anyhow, Context, Result};

fn main() {
	if let Err(e) = init() {
		fatal!(target: "builder", "{}", e);
	}
}

fn init() -> Result<()> {
	use ipc::Message;

	let mut ipc = ipc::Client::new(".zeus.sock")
		.context("Unable to connect to zeus")?;

	let gopts;
	if let Message::Init(opts) = ipc
		.recv()
		.context("Unable to receive initialization packet")?
	{
		use config::Color;
		match opts.color {
			Color::Always => log::set_color_enabled(true),
			Color::Never => log::set_color_enabled(false),
			_ => {},
		}

		set_log_level!(opts.log_level.clone());

		gopts = opts;
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
		Message::Sync(opts) => sync::sync(db, gopts, opts),
		Message::Remove(opts) => remove::remove(db, gopts, opts),
		p => Err(anyhow!(
			"This is a bug! Received unexpected packet: {p:#?}"
		)),
	}?;

	ipc.send(Message::Response(r))
		.context("Unable to send response")?;

	Ok(())
}

use config::GlobalOptions;
use db::Transaction;
use ipc::Response;

mod sync {
	use super::*;
	use config::SyncOptions;

	pub fn sync(
		mut db: db::DbGuard,
		gopts: GlobalOptions,
		opts: SyncOptions,
	) -> Result<Response> {
		let mut res = Response::default();

		for pkg_name in &opts.packages {
			match sync_package(
				&mut res, pkg_name, &mut db, &gopts, &opts,
			) {
				Ok(_) => {},
				Err(e) => {
					error!("{}", e);
					continue;
				},
			}
		}

		Ok(res)
	}

	fn sync_package(
		res: &mut Response,
		name: &str,
		db: &mut db::DbGuard,
		gopts: &GlobalOptions,
		opts: &SyncOptions,
	) -> Result<()> {
		let trans = Transaction::new()
			.clone_pkg(
				name,
				format!("{}/{}.git", gopts.aur_url, name),
				opts.upgrade,
			)
			.build_pkg(name, opts.build_args.iter());

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
			_ => warning!(
				"Package {name} synced but is not found in database"
			),
		}

		Ok(())
	}
}

mod remove {
	use super::*;
	use config::RemoveOptions;

	pub fn remove(
		mut db: db::DbGuard,
		_gopts: GlobalOptions,
		opts: RemoveOptions,
	) -> Result<Response> {
		let mut transaction = Transaction::new();

		for pkg in &opts.packages {
			transaction = transaction.remove_pkg(pkg);
		}

		db.commit(transaction)
			.context("Unable to remove all packages")?;

		let res = Response {
			packages: opts.packages,
			..Default::default()
		};

		Ok(res)
	}
}

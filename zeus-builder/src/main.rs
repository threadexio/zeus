#![deny(clippy::correctness)]
#![warn(
	clippy::all,
	clippy::style,
	clippy::complexity,
	clippy::perf,
	clippy::unwrap_used
)]

use anyhow::{anyhow, Context, Result};

use zeus_db::Db;
use zeus_ipc::Response;
use zeus_ipc::{Client, Message};
use zeus_term::Terminal;
use zeus_types::{Color, GlobalConfig, RemoveConfig, SyncConfig};

fn main() {
	{
		use nix::sys::stat::{umask, Mode};
		umask(Mode::S_IRWXO);
	}

	let mut term = Terminal::new();

	if let Err(e) = init(&mut term) {
		term.fatal(format!("{:#}", e));
	}
}

fn init(term: &mut Terminal) -> Result<()> {
	let mut ipc = Client::new(".zeus.sock")
		.context("Unable to connect to zeus")?;

	let global_config;
	if let Message::Init(config) = ipc
		.recv()
		.context("Unable to receive initialization packet")?
	{
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

	let mut db = Db::new(
		std::env::current_dir()
			.context("Unable to get current directory")?,
	)
	.context("Unable to initialize database")?;

	db.lock(global_config.db_key)
		.context("Cannot obtain shared lock on database")?;

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

mod sync {
	use super::*;

	pub fn sync(
		global_config: GlobalConfig,
		config: SyncConfig,
		term: &mut Terminal,
		mut db: Db,
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
		_: &mut Terminal,
		db: &mut Db,
		name: &str,
		res: &mut Response,
	) -> Result<()> {
		let mut package = if !db.exists(name) {
			db.clone(
				name,
				format!("{}/{}.git", global_config.aur_url, name),
			)
			.with_context(|| {
				format!("Unable to clone package '{name}'")
			})?
		} else {
			db.package(name).with_context(|| {
				format!("Unable to retrieve local package '{name}'")
			})?
		};

		if config.upgrade {
			package.update().with_context(|| {
				format!("Unable to update package '{name}'")
			})?;
		}

		package.build(config.build_args.iter()).with_context(
			|| format!("Unable to build package '{name}'"),
		)?;

		res.packages.push(name.to_string());
		res.files.extend(
			package
				.files()
				.context(format!(
					"Unable to get package files for '{name}'"
				))?
				.drain(..)
				.filter_map(|x| {
					x.strip_prefix(db.path())
						.map(|x| x.to_path_buf())
						.ok()
				}),
		);

		Ok(())
	}
}

mod remove {
	use super::*;

	pub fn remove(
		_: GlobalConfig,
		config: RemoveConfig,
		_: &mut Terminal,
		db: Db,
	) -> Result<Response> {
		for mut package in
			config.packages.iter().filter_map(|x| db.package(x).ok())
		{
			let name = package.name().to_string();
			package.remove().with_context(|| {
				format!("Unable to remove package '{name}'",)
			})?
		}

		let res = Response {
			packages: config.packages,
			..Default::default()
		};

		Ok(res)
	}
}

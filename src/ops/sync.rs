use super::prelude::*;
use super::start_builder;

pub fn sync(
	term: &mut Terminal,
	runtime: &mut Runtime,
	build_cache: &mut BuildCache,
	cfg: Config,
	opts: &mut SyncOptions,
) -> Result<()> {
	if opts.packages.is_empty() && opts.upgrade {
		opts.packages = build_cache
			.list_packages()?
			.iter()
			.map(|x| Package {
				name: x.to_owned(),
				..Default::default()
			})
			.collect();
	}

	if opts.packages.is_empty() {
		return Err(other!("No packages found"));
	}

	opts.packages = err!(
		cfg.aur.info(opts.packages.iter().map(|x| &x.name)),
		"Cannot request info for packages"
	)
	.results;

	if opts.packages.is_empty() {
		return Err(other!("No valid packages found"));
	}

	term.list(
		format!(
			"The following packages will be {}:",
			match opts.upgrade {
				true => "UPGRADED",
				false => "SYNCED",
			}
			.bold()
		),
		opts.packages.iter().map(|x| &x.name),
		4,
	)?;

	if !term.yes_no_question(
		"Are you sure you want to sync these packages?",
		true,
	)? {
		error!("Aborting...");
		return Ok(());
	}

	let synced_packages = start_builder(
		runtime,
		build_cache,
		&cfg,
		&opts.machine_name,
	)?;

	if opts.install {
		use std::process::Command;

		// do a quick path translation /build -> build_dir
		for p in &synced_packages {
			err!(
				Command::new("sudo")
					.args(["pacman", "-U"])
					.args(p.files.iter())
					.status(),
				"Failed to execute pacman"
			);
		}
	} else {
		term.list(
			"Synced packages:",
			synced_packages.iter().map(|x| &x.package.name),
			1,
		)?;
	}

	Ok(())
}

use super::prelude::*;
use super::start_builder;

pub fn sync(
	term: &mut Terminal,
	runtime: &mut Runtime,
	build_cache: &mut BuildCache,
	gopts: &mut GlobalOptions,
	mut opts: SyncOptions,
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
		gopts.aur.info(opts.packages.iter().map(|x| &x.name)),
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
		&Config {
			global_opts: gopts.clone(),
			operation: Operation::Sync(opts.clone()),
		},
		&opts.machine_name,
	)?;

	if opts.install {
		use std::path::Path;
		use std::process::Command;

		err!(
			Command::new("sudo")
				.args(["pacman", "-U"])
				.args(
					synced_packages // quick path translation from /build -> build cache
						.iter()
						.map(|x| x.files.iter())
						.flatten()
						.map(|x| {
							build_cache.path().join(
								Path::new(x)
									.strip_prefix("/build")
									.expect(
									"package file was not in /build",
								),
							)
						})
				)
				.status(),
			"Failed to execute pacman"
		);
	} else {
		term.list(
			"Synced packages:",
			synced_packages.iter().map(|x| &x.package.name),
			1,
		)?;
	}

	Ok(())
}

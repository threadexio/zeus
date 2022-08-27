use super::prelude::*;

pub fn remove(
	term: &mut Terminal,
	runtime: &mut Runtime,
	build_cache: &mut BuildCache,
	cfg: Config,
	opts: &mut RemoveOptions,
) -> Result<()> {
	let mut valid_packages: Vec<Package> = Vec::new();

	let synced_packages = build_cache.list_packages()?;
	for p in opts.packages.iter().map(|x| &x.name) {
		if synced_packages.contains(&p) {
			valid_packages.push(Package {
				name: p.to_owned(),
				..Default::default()
			})
		}
	}

	if valid_packages.is_empty() {
		return Err(other!("No packages found"));
	}

	opts.packages = valid_packages;

	term.list(
		format!(
			"The following packages will be {}:",
			"REMOVED".bold()
		),
		opts.packages.iter().map(|x| &x.name),
		4,
	)?;

	if !term.yes_no_question(
		"Are you sure you want to remove these packages?",
		true,
	)? {
		error!("Aborting...");
		return Ok(());
	}

	let removed_packages = start_builder(
		runtime,
		build_cache,
		&cfg,
		&opts.machine_name,
	)?;

	if opts.uninstall {
		use std::process::Command;

		err!(
			Command::new("sudo")
				.args(["pacman", "-R", "-c", "-s", "-n"])
				.args(
					removed_packages.iter().map(|x| &x.package.name)
				)
				.status(),
			"Failed to execute pacman"
		);
	} else {
		term.list(
			"Removed packages:",
			removed_packages.iter().map(|x| &x.package.name),
			1,
		)?;
	}

	Ok(())
}

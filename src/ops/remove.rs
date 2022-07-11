use std::collections::HashSet;
use std::path::Path;

use crate::ops::prelude::*;

use super::start_builder;

pub fn remove(
	term: &mut Terminal,
	runtime: &mut Runtime,
	mut cfg: AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	cfg.machine = args.value_of("name").unwrap().to_owned();

	cfg.packages = args
		.values_of("packages")
		.unwrap_or_default()
		.map(|x| x.to_owned())
		.collect();

	if cfg.packages.is_empty() {
		return Err(ZeusError::new(
			"zeus".to_owned(),
			"No packages specified.".to_owned(),
		));
	}

	let mut valid_packages: HashSet<String> = HashSet::new();
	let mut invalid_packages: HashSet<String> = HashSet::new();

	for pkg in cfg.packages {
		let pkg_path = Path::new(&cfg.build_dir).join(&pkg);

		if !pkg_path.exists()
			|| !pkg_path.is_dir()
			|| !pkg_path.join("PKGBUILD").exists()
		{
			invalid_packages.insert(pkg);
		} else {
			valid_packages.insert(pkg);
		}
	}
	cfg.packages = valid_packages;

	if !invalid_packages.is_empty() {
		term.list(
			format!(
				"The following packages have {} been synced:",
				"NOT".bold()
			),
			invalid_packages.iter(),
			4,
		)?;
	}

	if cfg.packages.is_empty() {
		return Err(ZeusError::new(
			"zeus".to_owned(),
			"No valid packages specified.".to_owned(),
		));
	}

	term.list(
		format!(
			"The following packages will be {}:",
			"REMOVED".bold()
		),
		cfg.packages.iter(),
		4,
	)?;

	if !term.yes_no_question(
		"Are you sure you want to remove these packages?",
		true,
	)? {
		error!(term.log, "zeus", "Aborting...");
		return Ok(());
	}

	let removed_packages = start_builder(term, runtime, cfg)?;

	term.list("Synced packages:", removed_packages.iter(), 1)?;

	Ok(())
}

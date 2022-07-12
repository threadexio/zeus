use std::collections::HashSet;
use std::fs;

use super::prelude::*;

use super::start_builder;

pub fn sync(
	term: &mut Terminal,
	runtime: &mut Runtime,
	mut cfg: AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	cfg.upgrade = args.is_present("upgrade");

	cfg.build_args = args
		.value_of("buildargs")
		.unwrap_or_default()
		.split_ascii_whitespace()
		.map(|x| x.to_owned())
		.collect();

	cfg.machine = args.value_of("name").unwrap().to_owned();

	cfg.packages = args
		.values_of("packages")
		.unwrap_or_default()
		.map(|x| x.to_owned())
		.collect();

	if cfg.packages.is_empty() && cfg.upgrade {
		for pkg in args.values_of("packages").unwrap_or_default() {
			cfg.packages.insert(pkg.to_owned());
		}

		let packages: Vec<String> = zerr!(
			fs::read_dir(&cfg.build_dir),
			"fs",
			"Cannot list {}",
			&cfg.build_dir
		)
		.filter_map(|x| x.ok())
		.filter(|x| x.path().is_dir())
		.map(|x| x.file_name().into_string())
		.filter_map(|x| x.ok())
		.collect();

		match term.question(
			"Choose which packages to upgrade:",
			packages.iter().map(|x| x.as_str()).collect(),
			"all",
			4,
		)? {
			None => {
				for package in packages {
					cfg.packages.insert(package);
				}
			},
			Some(answer) => {
				for package in answer {
					cfg.packages.insert(package.to_owned());
				}
			},
		}
	}

	if cfg.packages.is_empty() {
		return Err(ZeusError::new(
			"zeus".to_owned(),
			"No packages specified.".to_owned(),
		));
	}

	let pkg_info = zerr!(
		cfg.aur.info(&cfg.packages),
		"AUR",
		"Cannot request info for packages"
	);

	let mut valid_packages: HashSet<String> = HashSet::new();
	for pkg in pkg_info.results {
		if let Some(name) = pkg.Name {
			valid_packages.insert(name);
		}
	}

	let invalid_packages: Vec<&String> =
		cfg.packages.difference(&valid_packages).collect();
	if !invalid_packages.is_empty() {
		term.list(
			format!(
				"The following packages do {} exist in the AUR:",
				"NOT".bold()
			),
			invalid_packages.iter(),
			4,
		)?;
	}
	cfg.packages = valid_packages;

	if cfg.packages.is_empty() {
		return Err(ZeusError::new(
			"zeus".to_owned(),
			"No valid packages specified.".to_owned(),
		));
	}

	term.list(
		format!(
			"The following packages will be {}:",
			match cfg.upgrade {
				true => "UPGRADED",
				false => "SYNCED",
			}
			.bold()
		),
		cfg.packages.iter(),
		4,
	)?;

	if !term
		.yes_no_question("Are you sure you want to continue?", true)?
	{
		error!(term.log, "zeus", "Aborting...");
		return Ok(());
	}

	let synced_packages = start_builder(term, runtime, cfg)?;

	term.list("Synced packages:", synced_packages.iter(), 1)?;

	Ok(())
}

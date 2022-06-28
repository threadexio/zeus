use crate::ops::prelude::*;
use std::fs;

pub fn sync(
	term: &mut Terminal,
	runtime: &mut Runtime,
	cfg: &mut AppConfig,
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
			&format!("Cannot list {}", &cfg.build_dir)
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

		if cfg.packages.is_empty() {
			return Err(ZeusError::new(
				"zeus".to_owned(),
				"No packages specified. Exiting...".to_owned(),
			));
		}
	} else if cfg.packages.is_empty() {
		return Err(ZeusError::new(
			"zeus".to_owned(),
			"No packages specified.".to_owned(),
		));
	}

	debug!(term.log, "debug", "{:?}", &cfg);

	if !term.yes_no_question(
		match cfg.upgrade {
			true => {
				"Are you sure you want to upgrade these packages?"
			},
			false => "Are you sure you want to build these packages?",
		},
		true,
	)? {
		error!(term.log, "zeus", "Aborting...");
		return Ok(());
	}

	// start machine

	// send data to machine

	// attach to machine and display build progress

	todo!()
}

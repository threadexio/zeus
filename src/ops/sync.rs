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
	cfg.install = args.is_present("install");

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
		.map(|x| Package {
			Name: Some(x.to_owned()),
			..Default::default()
		})
		.collect();

	if cfg.packages.is_empty() && cfg.upgrade {
		// CHANGELOG: Simplify code

		let dir = zerr!(
			fs::read_dir(&cfg.build_dir),
			"fs",
			"Cannot list {}",
			&cfg.build_dir
		);

		for entry in dir {
			let entry = match entry {
				Err(e) => {
					warning!(
						"fs",
						"Cannot read package directory: {}",
						e
					);
					continue;
				},
				Ok(v) => v,
			};

			if entry.path().is_dir() {
				match entry.file_name().into_string() {
					Ok(v) => cfg.packages.push(Package {
						Name: Some(v),
						..Default::default()
					}),
					Err(e) => {
						warning!(
							"fs",
							"Found invalid package directory: {}",
							e.to_string_lossy()
						);
						continue;
					},
				};
			}
		}

		// CHANGELOG: dont ask what to upgrade
	}

	if cfg.packages.is_empty() {
		return Err(ZeusError::new(
			"zeus".to_owned(),
			"No packages found.".to_owned(),
		));
	}

	// CHANGELOG: remove invalid packages

	cfg.packages = zerr!(
		cfg.aur.info(
			&cfg.packages
				.iter()
				.filter_map(|x| x.Name.as_ref())
				.collect()
		),
		"AUR",
		"Cannot request info for packages"
	)
	.results;

	if cfg.packages.is_empty() {
		return Err(ZeusError::new(
			"zeus".to_owned(),
			"No valid packages found.".to_owned(),
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
		cfg.packages.iter().filter_map(|x| x.Name.as_ref()),
		4,
	)?;

	if !term.yes_no_question(
		"Are you sure you want to sync these packages?",
		true,
	)? {
		error!("zeus", "Aborting...");
		return Ok(());
	}

	let mut synced_packages = start_builder(runtime, &cfg)?;

	if cfg.install {
		use std::process::Command;

		let mut package_files: Vec<String> = vec![];

		for p in &mut synced_packages {
			if let Some(pkg_files) = &mut p.package_files {
				package_files.append(
					&mut pkg_files
						.iter()
						.filter_map(|x| {
							if let Some(s) = x.strip_prefix("/build")
							{
								Some(format!(
									"{}/{}",
									&cfg.build_dir, s
								))
							} else {
								None
							}
						})
						.collect(),
				);
			}
		}

		if package_files.is_empty() {
			info!("zeus", "Nothing to install");
			return Ok(());
		}

		zerr!(
			Command::new("sudo")
				.args(["pacman", "-U"])
				.args(package_files)
				.status(),
			"zeus",
			"Failed to execute pacman"
		);
	} else {
		term.list(
			"Synced packages:",
			synced_packages.iter().filter_map(|x| x.Name.as_ref()),
			1,
		)?;
	}

	Ok(())
}

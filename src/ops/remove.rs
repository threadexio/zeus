use std::path::Path;

use super::prelude::*;

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
		.map(|x| Package {
			Name: Some(x.to_owned()),
			..Default::default()
		})
		.filter(|x| {
			let pkg_path = Path::new(&cfg.build_dir)
				.join(x.Name.as_ref().unwrap());

			if pkg_path.exists()
				&& pkg_path.is_dir()
				&& pkg_path.join("PKGBUILD").exists()
			{
				true
			} else {
				warning!(
					"zeus",
					"Package {} was not found",
					x.Name.as_ref().unwrap()
				);
				false
			}
		})
		.collect();

	if cfg.packages.is_empty() {
		return Err(ZeusError::new(
			"zeus".to_owned(),
			"No packages found.".to_owned(),
		));
	}

	term.list(
		format!(
			"The following packages will be {}:",
			"REMOVED".bold()
		),
		cfg.packages.iter().filter_map(|x| x.Name.as_ref()),
		4,
	)?;

	if !term.yes_no_question(
		"Are you sure you want to remove these packages?",
		true,
	)? {
		error!("zeus", "Aborting...");
		return Ok(());
	}

	let removed_packages = start_builder(runtime, cfg)?;

	term.list(
		"Removed packages:",
		removed_packages.iter().filter_map(|x| x.Name.as_ref()),
		1,
	)?;

	Ok(())
}

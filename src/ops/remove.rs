use crate::ops::prelude::*;

pub fn remove(
	term: &mut Terminal,
	runtime: &mut Runtime,
	cfg: &mut AppConfig,
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

	term.list(
		"The following packages will be REMOVED:",
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

	// start machine

	// send data to machine

	// attach to machine and display build progress

	todo!()
}

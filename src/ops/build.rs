use crate::ops::prelude::*;

pub fn build(
	term: &mut Terminal,
	runtime: &mut Runtime,
	mut cfg: AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	cfg.image = args.value_of("image").unwrap().to_owned();
	cfg.machine = args.value_of("name").unwrap().to_owned();

	for machine in runtime.list_machines()? {
		if machine == cfg.machine {
			debug!(
				term.log,
				"MachineManager",
				"Removing old machine {}",
				cfg.machine
			);
			runtime.delete_machine(&machine)?;
		}
	}

	debug!(term.log, "ImageManager", "Updating image {}", cfg.image);
	runtime.make_image(&cfg.image)?;

	debug!(
		term.log,
		"MachineManager", "Creating new machine {}", cfg.machine
	);
	runtime.create_machine(&cfg.machine, &cfg.image, &cfg)?;

	Ok(())
}

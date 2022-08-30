use super::prelude::*;

pub fn build(
	runtime: &mut Runtime,
	mut cfg: AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	cfg.image = args.value_of("image").unwrap().to_owned();
	cfg.machine = args.value_of("name").unwrap().to_owned();

	for machine in runtime.list_machines()? {
		if machine == cfg.machine {
			debug!(
				"MachineManager",
				"Removing old machine {}", cfg.machine
			);
			runtime.delete_machine(&machine)?;
		}
	}

	debug!("ImageManager", "Updating image {}", cfg.image);
	runtime.make_image(&cfg.image)?;

	debug!("MachineManager", "Creating new machine {}", cfg.machine);
	runtime.create_machine(&cfg.machine, &cfg.image, &cfg)?;

	Ok(())
}

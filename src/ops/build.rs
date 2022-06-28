use crate::ops::prelude::*;

pub fn build(
	term: &mut Terminal,
	runtime: &mut Runtime,
	cfg: &mut AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	cfg.image = args.value_of("image").unwrap().to_owned();
	cfg.machine = args.value_of("name").unwrap().to_owned();

	debug!(term.log, "post-op config", "{:?}", &cfg);

	debug!(
		term.log,
		"MachineManager", "Removing old machine {}", cfg.machine
	);
	for machine in runtime.list_machines().unwrap() {
		if machine.name() == cfg.machine {
			runtime.delete_machine(machine).unwrap();
		}
	}

	debug!(term.log, "ImageManager", "Updating image {}", cfg.image);
	let image = runtime.create_image(&cfg.image).unwrap();

	debug!(
		term.log,
		"MachineManager", "Creating new machine {}", cfg.machine
	);
	runtime
		.create_machine(&cfg.machine, image.as_ref(), cfg)
		.unwrap();

	Ok(())
}

use super::prelude::*;

pub fn build(
	global_config: GlobalConfig,
	_: BuildConfig,
	runtime: &mut Runtime,
) -> Result<()> {
	debug!("Getting machine list");

	for machine in
		runtime.list_machines().context("Unable to list machines")?
	{
		if machine == global_config.machine_name {
			debug!("Deleting old machine");
			runtime
				.delete_machine(&machine)
				.context("Unable to delete machine")?;
		}
	}

	debug!("Making new image");

	runtime
		.make_image(&global_config.machine_image)
		.context("Unable to make image")?;

	debug!("Creating new machine from image");

	runtime
		.create_machine(
			&global_config.machine_name,
			&global_config.machine_image,
			&global_config,
		)
		.context("Unable to create new builder")?;

	Ok(())
}

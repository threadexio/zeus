use super::prelude::*;

pub fn build(
	runtime: &mut Runtime,
	gopts: GlobalOptions,
) -> Result<()> {
	debug!("Getting machine list");

	for machine in
		runtime.list_machines().context("Unable to list machines")?
	{
		if machine == gopts.machine_name {
			debug!("Deleting old machine");
			runtime
				.delete_machine(&machine)
				.context("Unable to delete machine")?;
		}
	}

	debug!("Making new image");

	runtime
		.make_image(&gopts.machine_image)
		.context("Unable to make image")?;

	debug!("Creating new machine from image");

	runtime
		.create_machine(
			&gopts.machine_name,
			&gopts.machine_image,
			&gopts,
		)
		.context("Unable to create new builder")?;

	Ok(())
}

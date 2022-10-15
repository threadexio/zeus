use super::prelude::*;

pub fn build(
	runtime: &mut Runtime,
	gopts: GlobalOptions,
	_opts: BuildOptions,
) -> Result<()> {
	for machine in
		runtime.list_machines().context("Unable to list machines")?
	{
		if machine == gopts.machine_name {
			runtime
				.delete_machine(&machine)
				.context("Unable to delete machine")?;
		}
	}

	runtime
		.make_image(&gopts.machine_image)
		.context("Unable to make image")?;

	runtime
		.create_machine(
			&gopts.machine_name,
			&gopts.machine_image,
			&gopts,
		)
		.context("Unable to create new builder")?;

	Ok(())
}

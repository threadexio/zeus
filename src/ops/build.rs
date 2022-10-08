use super::prelude::*;

pub fn build(
	runtime: &mut BoxedRuntime,
	gopts: GlobalOptions,
	opts: BuildOptions,
) -> Result<()> {
	for machine in
		runtime.list_machines().context("Unable to get machines")?
	{
		if machine == opts.machine_name {
			runtime
				.delete_machine(&machine)
				.context("Unable to delete machine")?;
		}
	}

	runtime
		.make_image(&opts.machine_image)
		.context("Unable to make image")?;

	runtime
		.create_machine(
			&opts.machine_name,
			&opts.machine_image,
			&gopts,
		)
		.context("Unable to create new builder")?;

	Ok(())
}

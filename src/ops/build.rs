use super::prelude::*;

pub fn build(
	runtime: &mut Runtime,
	gopts: &mut GlobalOptions,
	opts: BuildOptions,
) -> Result<()> {
	for machine in
		runtime.list_machines().map_err(|x| other!("{}", x))?
	{
		if machine == opts.machine_name {
			debug!("Removing old machine {}", &opts.machine_name);
			runtime
				.delete_machine(&machine)
				.map_err(|x| other!("{}", x))?;
		}
	}

	debug!("Updating image {}", &opts.machine_image);
	runtime
		.make_image(&opts.machine_image)
		.map_err(|x| other!("{}", x))?;

	debug!("Creating new machine {}", &opts.machine_name);
	runtime
		.create_machine(
			&opts.machine_name,
			&opts.machine_image,
			gopts,
		)
		.map_err(|x| other!("{}", x))?;

	Ok(())
}

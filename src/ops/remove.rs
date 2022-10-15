use super::prelude::*;

pub fn remove(
	runtime: &mut Runtime,
	pstore: &mut PackageStore,
	gopts: GlobalOptions,
	mut opts: RemoveOptions,
) -> Result<()> {
	if opts.packages.is_empty() {
		opts.packages = inquire::MultiSelect::new(
			"Select packages to remove:",
			pstore.list()?.drain(..).map(|x| x.name).collect(),
		)
		.prompt()?
		.drain(..)
		.map(|x| Package { name: x, ..Default::default() })
		.collect();
	} else {
		opts.packages.retain(|x| match pstore.exists(&x.name) {
			Ok(v) => v,
			Err(_) => {
				warn!("{}: Not synced", &x.name);
				false
			},
		});
	}

	if opts.packages.is_empty() {
		return Err(Error::new("No valid packages specified"));
	}

	if !inquire::Confirm::new("Proceed to remove packages?")
		.with_default(true)
		.prompt()?
	{
		return Err(Error::new("Aborting..."));
	}

	super::start_builder(
		runtime,
		pstore,
		&gopts,
		Operation::Remove(opts),
	)
	.context("Unable to start builder")?;

	// TODO: Handle --uninstall

	Ok(())
}

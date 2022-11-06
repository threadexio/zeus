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
			pstore
				.list_pkgs()?
				.drain(..)
				.map(|x| x.name().to_string())
				.collect(),
		)
		.prompt()?;
	} else {
		opts.packages.retain(|x| {
			if pstore.get_pkg(x).is_none() {
				true
			} else {
				warn!("{}: Not synced", x);
				false
			}
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

	let removed_packages = super::start_builder(
		runtime,
		pstore,
		&gopts,
		Operation::Remove(opts.clone()),
	)
	.context("Unable to start builder")?;

	if removed_packages.is_empty() {
		return Err(Error::new("No packages removed!"));
	}

	if opts.uninstall {
		let status = tools::Pacman::default()
			.attach(true)
			.remove()
			.cascade()
			.recursive()
			.args(&removed_packages)
			.wait()
			.context("Unable to run pacman")?
			.status;

		if !status.success() {
			return Err(Error::new(
				"Failed to uninstall packages with pacman",
			));
		}
	}

	Ok(())
}

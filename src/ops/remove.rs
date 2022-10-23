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
		opts.packages.retain(|x| {
			if pstore.exists(&x.name) {
				true
			} else {
				warn!("{}: Not synced", &x.name);
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

	let removed_packages: Vec<Package> = super::start_builder(
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
		let status = std::process::Command::new("sudo")
			.args(["--", "pacman", "-R", "-c", "-n", "-s", "--"])
			.args(removed_packages.iter().map(|x| x.name.as_str()))
			.status()
			.context("Unable to run pacman")?;

		if !status.success() {
			return Err(Error::new(
				"Failed to uninstall packages with pacman",
			));
		}
	}

	Ok(())
}

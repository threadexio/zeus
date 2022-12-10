use super::prelude::*;

pub fn remove(
	runtime: &mut Runtime,
	db: db::DbGuard,
	gopts: GlobalOptions,
	mut opts: RemoveOptions,
) -> Result<()> {
	if opts.packages.is_empty() {
		error!("No packages specified");
		return Ok(());
	}

	opts.packages.retain(|x| {
		if db.pkg(x).is_ok() {
			true
		} else {
			error!("Package {} is not synced", x);
			false
		}
	});

	if opts.packages.is_empty() {
		error!("No valid packages specified");
		return Ok(());
	}

	if !inquire::Confirm::new("Proceed to remove packages?")
		.with_default(true)
		.prompt()?
	{
		info!("Aborting...");
		return Ok(());
	}

	let res = super::start_builder(
		runtime,
		gopts,
		Message::Remove(opts.clone()),
	)
	.context("Unable to start builder")?;

	trace!("removed packages: {:#?}", &res.packages);

	if opts.uninstall {
		let status = db::tools::Pacman::default()
			.attach(true)
			.remove()
			.cascade()
			.recursive()
			.args(res.packages)
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

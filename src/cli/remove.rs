use super::prelude::*;
use ipc::Message;

pub fn remove(
	global_config: GlobalConfig,
	mut config: RemoveConfig,
	runtime: &mut Runtime,
	db: db::DbGuard,
) -> Result<()> {
	if config.packages.is_empty() {
		bail!("No packages specified")
	}

	for pkg in &config.packages {
		db.pkg(pkg).with_context(|| {
			format!("Package '{pkg}' not found in database")
		})?;
	}

	if !inquire::Confirm::new("Proceed to remove packages?")
		.with_default(true)
		.prompt()?
	{
		info!("Aborting...");
		return Ok(());
	}

	let res = super::start_builder(
		global_config,
		Message::Remove(config.clone()),
		runtime,
	)
	.context("Unable to start builder")?;

	trace!("removed packages: {:#?}", &res.packages);

	if config.uninstall {
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
			bail!("Unable to uninstall packages with pacman",);
		}
	}

	Ok(())
}

use super::prelude::*;
use ipc::Message;

pub fn remove(
	global_config: GlobalConfig,
	mut config: RemoveConfig,
	runtime: &mut Runtime,
	db: db::DbGuard,
) -> Result<()> {
	if config.packages.is_empty() {
		error!("No packages specified");
		return Ok(());
	}

	config.packages.retain(|x| {
		if db.pkg(x).is_ok() {
			true
		} else {
			error!("Package {} is not synced", x);
			false
		}
	});

	if config.packages.is_empty() {
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
			return Err(anyhow!(
				"Failed to uninstall packages with pacman",
			));
		}
	}

	Ok(())
}

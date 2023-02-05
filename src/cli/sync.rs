use super::prelude::*;
use ipc::Message;

pub fn sync(
	global_config: GlobalConfig,
	mut config: SyncConfig,
	runtime: &mut Runtime,
	db: db::DbGuard,
	aur: &mut aur::Aur,
) -> Result<()> {
	if config.upgrade && config.packages.is_empty() {
		config.packages = db
			.list_pkgs()
			.context("Unable to list local packages")?
			.drain(..)
			.map(|x| x.name().to_string())
			.collect();
	}

	if config.packages.is_empty() {
		error!("No packages specified");
		return Ok(());
	}

	let mut packages = aur
		.info(config.packages.iter())
		.context("Unable to request package data from AUR")?;

	if packages.len() > config.packages.len() {
		warning!("AUR returned more packages than requested. This might be a bug with zeus or the AUR!");
	}

	config.packages = packages.drain(..).map(|x| x.name).collect();

	if config.packages.is_empty() {
		error!("No valid packages specified");
		return Ok(());
	}

	if !inquire::Confirm::new(&format!(
		"Proceed to {} {} packages? {}",
		if config.upgrade {
			"upgrade"
		} else {
			"sync"
		},
		packages.len(),
		config.packages.join(" ").bold()
	))
	.with_default(true)
	.prompt()?
	{
		info!("Aborting...");
		return Ok(());
	}

	let res = super::start_builder(
		global_config,
		Message::Sync(config.clone()),
		runtime,
	)
	.context("Unable to start builder")?;

	trace!("synced packages: {:#?}", &res.packages);

	if config.install {
		let status = db::tools::Pacman::default()
			.attach(true)
			.upgrade()
			.args(res.files.iter().map(|x| db.root().join(x)))
			.wait()
			.context("Unable to run pacman")?
			.status;

		if !status.success() {
			return Err(anyhow!(
				"Failed to install packages with pacman",
			));
		}
	}

	Ok(())
}

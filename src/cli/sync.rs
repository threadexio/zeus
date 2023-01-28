use super::prelude::*;

pub fn sync(
	runtime: &mut Runtime,
	db: db::DbGuard,
	aur: &mut aur::Aur,
	gopts: GlobalOptions,
	mut opts: SyncOptions,
) -> Result<()> {
	if opts.upgrade && opts.packages.is_empty() {
		opts.packages = db
			.list_pkgs()
			.context("Unable to list local packages")?
			.drain(..)
			.map(|x| x.name().to_string())
			.collect();
	}

	if opts.packages.is_empty() {
		error!("No packages specified");
		return Ok(());
	}

	let mut packages = aur
		.info(opts.packages.iter())
		.context("Unable to request package data from AUR")?;

	if packages.len() > opts.packages.len() {
		warning!("AUR returned more packages than requested. This might be a bug with zeus or the AUR!");
	}

	opts.packages = packages.drain(..).map(|x| x.name).collect();

	if opts.packages.is_empty() {
		error!("No valid packages specified");
		return Ok(());
	}

	if !inquire::Confirm::new(&format!(
		"Proceed to {} {} packages? {}",
		if opts.upgrade {
			"upgrade"
		} else {
			"sync"
		},
		packages.len(),
		opts.packages.join(" ").bold()
	))
	.with_default(true)
	.prompt()?
	{
		info!("Aborting...");
		return Ok(());
	}

	let res = super::start_builder(
		runtime,
		gopts,
		Message::Sync(opts.clone()),
	)
	.context("Unable to start builder")?;

	trace!("synced packages: {:#?}", &res.packages);

	if opts.install {
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

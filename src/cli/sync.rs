use super::prelude::*;

pub fn sync(
	runtime: &mut Runtime,
	db: &mut db::Db,
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
		return Err(Error::new("No packages specified"));
	}

	let mut packages = aur
		.info(opts.packages.iter())
		.context("Unable to request package data from AUR")?;

	if packages.len() > opts.packages.len() {
		warning!("AUR returned more packages than requested. This might be a bug with zeus or the AUR!");
	}

	opts.packages = packages.drain(..).map(|x| x.name).collect();

	if opts.packages.is_empty() {
		return Err(Error::new("No valid packages specified"));
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
		return Err(Error::new("Aborting..."));
	}

	let built_packages = super::start_builder(runtime)
		.context("Unable to start builder")?;

	if opts.install {
		let mut pacman =
			db::tools::Pacman::default().attach(true).upgrade();

		for p in built_packages.iter().filter_map(|x| db.get_pkg(x)) {
			match p.get_install_files().context(format!(
				"Unable to get package files for {}",
				p.name()
			)) {
				Ok(v) => {
					pacman = pacman.args(&v);
				},
				Err(e) => {
					warning!("{}", e);
				},
			}
		}

		let status =
			pacman.wait().context("Unable to run pacman")?.status;

		if !status.success() {
			return Err(Error::new(
				"Failed to install packages with pacman",
			));
		}
	}

	Ok(())
}

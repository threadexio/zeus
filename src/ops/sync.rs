use super::prelude::*;

use colored::Colorize;

pub fn sync(
	mut runtime: Runtime,
	pstore: &mut PackageStore,
	gopts: GlobalOptions,
	mut opts: SyncOptions,
) -> Result<()> {
	if opts.upgrade && opts.packages.is_empty() {
		opts.packages = pstore
			.list_pkgs()
			.context("Unable to list local packages")?
			.drain(..)
			.map(|x| x.name().to_string())
			.collect();
	}

	if opts.packages.is_empty() {
		return Err(Error::new("No packages specified"));
	}

	let mut res = gopts
		.aur
		.info(opts.packages.iter())
		.context("Unable to request package data from AUR")?;

	if res.result_count > opts.packages.len() {
		warn!("AUR returned more packages than requested. This might be a bug with zeus!")
	}

	opts.packages = res.results.drain(..).map(|x| x.name).collect();

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
		res.result_count,
		opts.packages.join(" ").bold()
	))
	.with_default(true)
	.prompt()?
	{
		return Err(Error::new("Aborting..."));
	}

	let built_packages = super::start_builder(
		&mut runtime,
		pstore,
		&gopts,
		Operation::Sync(opts.clone()),
	)
	.context("Unable to start builder")?;

	if opts.install {
		let mut pacman =
			tools::Pacman::default().attach(true).upgrade();

		for p in
			built_packages.iter().filter_map(|x| pstore.get_pkg(x))
		{
			match p.get_install_files().context(format!(
				"Unable to get package files for {}",
				p.name()
			)) {
				Ok(v) => {
					pacman = pacman.args(&v);
				},
				Err(e) => {
					warn!("{}", e);
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

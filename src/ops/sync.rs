use super::prelude::*;

use colored::Colorize;

pub fn sync(
	//runtime: &mut Runtime,
	pstore: &mut PackageStore,
	gopts: GlobalOptions,
	mut opts: SyncOptions,
) -> Result<()> {
	if opts.upgrade {
		if opts.packages.is_empty() {
			opts.packages = pstore.list()?;
		}
	}

	if opts.packages.is_empty() {
		return Err(Error::new("No packages specified"));
	}

	let res = gopts
		.aur
		.info(opts.packages.iter().map(|x| &x.name))
		.context("Unable to request package data from AUR")?;

	if res.result_count > opts.packages.len() {
		warn!("AUR returned more packages than requested. This might be a bug with zeus!")
	}

	opts.packages = res.results;

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
		opts.packages
			.iter()
			.map(|x| x.name.as_str())
			.collect::<Vec<_>>()
			.join(" ")
			.bold()
	))
	.with_default(true)
	.prompt()?
	{
		return Err(Error::new("Aborting..."));
	}

	// TODO: Start builder

	// TODO: Handle --install

	Ok(())
}

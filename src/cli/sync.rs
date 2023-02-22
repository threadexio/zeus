use super::prelude::*;
use ipc::Message;

pub(crate) fn sync(
	global_config: GlobalConfig,
	mut config: SyncConfig,
	term: &mut Terminal,
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
		bail!("No packages specified")
	}

	let mut packages = aur
		.info(config.packages.iter())
		.context("Unable to request package data from AUR")?;

	if packages.len() > config.packages.len() {
		term.warn("AUR returned more packages than requested. This might be a bug with zeus or the AUR!")?;
	}

	let packages: Vec<_> =
		packages.drain(..).map(|x| x.name).collect();

	config.packages.retain(|x| {
		if !packages.contains(x) {
			let _ = term.warn(format!("Package not found: {x}"));
			false
		} else {
			true
		}
	});

	if config.packages.is_empty() {
		bail!("No valid packages specified")
	}

	term.writeln(format!(
		"{} ({}):{}",
		"📦 Packages".bold(),
		config.packages.len(),
		config.packages.iter().fold(
			String::with_capacity(256),
			|mut a, x| {
				a.push_str(&format!("\n    {} {x}", "●".green()));
				a
			}
		)
	))?;

	if !term
		.confirm("Proceed with installation?".underline(), true)?
	{
		term.writeln("Aborting.".bold())?;
		return Ok(());
	}

	let res = super::start_builder(
		global_config,
		Message::Sync(config.clone()),
		term,
		runtime,
	)
	.context("Unable to start builder")?;

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

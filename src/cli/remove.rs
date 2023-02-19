use super::prelude::*;
use ipc::Message;

pub(crate) fn remove(
	global_config: GlobalConfig,
	config: RemoveConfig,
	term: &mut Terminal,
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

	term.writeln(format!(
		"{} ({}):\n    {}\n",
		"Packages".bold(),
		config.packages.len(),
		config.packages.join("\n    ").trim()
	))?;

	if !term.confirm("Do you want to remove these packages?", true)? {
		term.writeln("Aborting.".bold())?;
		return Ok(());
	}

	let res = super::start_builder(
		global_config,
		Message::Remove(config.clone()),
		term,
		runtime,
	)
	.context("Unable to start builder")?;

	term.trace(format!("removed packages: {:#?}", &res.packages))?;

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

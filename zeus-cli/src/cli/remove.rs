use super::prelude::*;
use zeus_ipc::Message;

pub(crate) fn remove(
	global_config: GlobalConfig,
	config: RemoveConfig,
	term: &mut Terminal,
	runtime: &mut Runtime,
	db: &mut Db,
) -> Result<()> {
	if config.packages.is_empty() {
		bail!("No packages specified")
	}

	for pkg in &config.packages {
		db.package(pkg).with_context(|| {
			format!("Package not found in database: {pkg}")
		})?;
	}

	term.writeln(format!(
		"{} ({}):{}",
		"📦 Packages".bold(),
		config.packages.len(),
		config.packages.iter().fold(
			String::with_capacity(256),
			|mut a, x| {
				a.push_str(&format!(
					"\n    {} {x}",
					"●".bright_green()
				));
				a
			}
		)
	));

	if !term.confirm(
		"Do you want to remove these packages?".underline(),
		true,
	) {
		term.writeln("Aborting.".bold());
		return Ok(());
	}

	let res = super::start_builder(
		global_config,
		Message::Remove(config.clone()),
		term,
		runtime,
	)
	.context("Unable to start builder")?;

	if config.uninstall {
		use std::process::{Command, Stdio};

		let status = Command::new("pacman")
			.stdin(Stdio::inherit())
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit())
			.arg("-R")
			.arg("-s")
			.args(res.packages)
			.output()
			.context("Unable to run pacman")?
			.status;

		if !status.success() {
			bail!("Unable to uninstall packages with pacman",);
		}
	}

	Ok(())
}

use crate::config;
use crate::error::{Result, ZeusError, ZeusError};
use crate::log::{self, Level};

use clap::ArgMatches;
use std::fs;
use std::path::Path;

pub async fn remove(
	logger: &mut log::Logger,
	cfg: &mut config::AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	logger.v(
		Level::Info,
		format!("Removing: \n{}", cfg.packages.join("\n")),
	);

	for pkg in &cfg.packages {
		let pkg_dir = Path::new(&cfg.builddir).join(pkg);

		if cfg.verbose {
			logger.v(
				Level::Verbose,
				format!(
					"Removing package: {}\nPackage directory: {}",
					pkg,
					&pkg_dir.to_string_lossy()
				),
			);
		}

		if !pkg_dir.exists() {
			logger.v(
				Level::Warn,
				format!("Package has not been synced: {}", pkg),
			);
			continue;
		}

		if !cfg.force {
			if !pkg_dir.is_dir() {
				logger.v(
					Level::Warn,
					format!(
						"Package directory is not a directory: {}",
						&pkg_dir.to_string_lossy()
					),
				);
				continue;
			}
		}

		ZeusError!(
			fs::remove_dir_all(pkg_dir),
			"Error removing package directory: "
		);
	}

	Ok(())
}

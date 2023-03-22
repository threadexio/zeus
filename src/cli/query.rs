use super::prelude::*;
use config::types::Output;

use std::io::Write;

macro_rules! print_info {
	($term:expr, $a:expr, $b:expr) => {
		let _ =
			$term.writeln(format!("{0: <16}: {1}", $a.bold(), $b));
	};

	(@option $term:expr, $a:expr, $b:expr) => {{
		match $b {
			None => {},
			Some(v) => {
				print_info!($term, $a, v);
			},
		}
	}};

	(@vec $term:expr, $a:expr, $b:expr) => {{
		match $b {
			Some(v) => {
				if !v.is_empty() {
					print_info!($term, $a, v.join(" "));
				}
			},
			None => {},
		}
	}};
}

fn print_pretty_package(term: &mut Terminal, package: &aur::Package) {
	print_info!(term, "Name", &package.name);
	print_info!(term, "Version", &package.version);
	print_info!(@option term, "Description", &package.description);
	print_info!(@option term, "URL", &package.url);
	print_info!(@option term, "Maintainer", &package.maintainer);
	print_info!(term, "Popularity", &package.popularity);
	print_info!(term, "Votes", &package.num_votes);

	print_info!(@vec term, "License", &package.license);
	print_info!(@vec term, "Groups", &package.groups);
	print_info!(@vec term, "Provides", &package.provides);
	print_info!(@vec term, "Depends On", &package.depends);
	print_info!(@vec term, "Optional Deps", &package.opt_depends);
	print_info!(@vec term, "Conflicts", &package.conflicts);
	print_info!(@vec term, "Replaces", &package.replaces);

	print_info!(term, "Last Modified", &package.last_modified);
	print_info!(term, "First Submitted", &package.first_submitted);

	print_info!(@option term, "Out Of Date", &package.out_of_date);

	println!();
}

fn format_timestamp(timestamp: u64) -> Option<String> {
	let d = chrono::NaiveDateTime::from_timestamp_opt(
		timestamp.try_into().ok()?,
		0,
	)?;

	Some(d.format("%c").to_string())
}

pub(crate) fn query(
	_: GlobalConfig,
	config: QueryConfig,
	term: &mut Terminal,
	db: &mut db::Db,
	aur: &mut aur::Aur,
) -> Result<()> {
	if config.keywords.is_empty() {
		let pkgs =
			db.list().context("Unable to get synced packages")?;

		match config.output {
			Output::Pretty => {
				for x in pkgs {
					writeln!(
						unsafe { term.raw_out() },
						"{}",
						x.name()
					)?;
				}
			},
			Output::Json => serde_json::to_writer(
				std::io::stdout(),
				&pkgs
					.map(|x| x.name().to_string())
					.collect::<Vec<_>>(),
			)
			.context("Unable to serialize JSON")?,
		}

		return Ok(());
	}

	let packages = match config.info {
		true => aur.info(config.keywords.iter()),
		false => aur.search(config.by, config.keywords.iter()),
	}
	.context("Unable to request packages from AUR")?;

	match config.output {
		Output::Json => serde_json::to_writer(
			unsafe { term.raw_out() },
			&packages,
		)
		.context("Unable to serialize JSON")?,
		Output::Pretty => {
			if config.info {
				for package in &packages {
					print_pretty_package(term, package);
				}
			} else {
				for package in &packages {
					term.writeln(format!(
						"{}{} {} {}{}{}",
						"aur/".bright_purple().bold(),
						package.name.bold(),
						package.version.bright_green().bold(),
						match package.out_of_date {
							Some(timestamp) => format!(
								" [out of date: {}] ",
								format_timestamp(timestamp)
									.unwrap_or("orphaned".to_string())
									.bold()
							),
							None => String::new(),
						}
						.bright_red(),
						match package.maintainer {
							None => " [orphan] ",
							Some(_) => "",
						}
						.bright_red()
						.bold(),
						match package.description {
							Some(ref desc) => format!("\n    {desc}"),
							None => String::new(),
						},
					));
				}
			}
		},
	}

	Ok(())
}

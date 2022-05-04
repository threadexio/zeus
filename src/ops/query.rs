use crate::aur;
use crate::config;
use crate::error::ZeusError;
use crate::log::{self, Level};

use clap::ArgMatches;

use std::io::stdout;

macro_rules! print_if_some {
	($a:expr,$b:expr) => {{
		match $b {
			None => {}
			Some(v) => {
				println!("{0: <16}: {1}", $a, v.join(" "));
			}
		}
	}};
}

fn print_pretty_package(package: &aur::Package) {
	println!("{0: <16}: {1}", "Name", &package.Name);
	println!("{0: <16}: {1}", "Version", &package.Version);
	println!("{0: <16}: {1}", "Description", &package.Description);
	println!("{0: <16}: {1}", "URL", &package.URL);

	print_if_some!("License", &package.License);
	print_if_some!("Groups", &package.Groups);
	print_if_some!("Provides", &package.Provides);
	print_if_some!("Depends On", &package.Depends);
	print_if_some!("Optional Deps", &package.OptDepends);
	print_if_some!("Conflicts", &package.Conflicts);
	print_if_some!("Replaces", &package.Replaces);

	println!(
		"{0: <16}: {1}",
		"Maintainer",
		match &package.Maintainer {
			Some(v) => v,
			None => "none",
		}
	);

	println!("{0: <16}: {1}", "Last Modified", &package.LastModified);
	println!("{0: <16}: {1}", "First Submitted", &package.FirstSubmitted);

	println!(
		"{0: <16}: {1}",
		"Out of date",
		match package.OutOfDate {
			Some(_) => "yes",
			None => "no",
		}
	);

	println!("{0: <16}: {1}", "Popularity", package.Popularity);
	println!("{0: <16}: {1}", "Votes", package.NumVotes);
}

pub async fn query(
	logger: &mut log::Logger,
	cfg: config::AppConfig,
	args: &ArgMatches,
) -> Result<(), ZeusError> {
	if cfg.keywords.is_empty() {
		return Err(ZeusError::new("aur", "No keywords specified".to_owned()));
	}

	let by = args.value_of_t::<aur::By>("by").unwrap();

	let res = match args.is_present("info") {
		true => cfg.aur.info(&cfg.keywords).await,
		false => cfg.aur.search(by, &cfg.keywords).await,
	};

	let data = match res {
		Ok(v) => v,
		Err(e) => {
			return Err(ZeusError::new("aur", format!("Error: {}", e)));
		}
	};

	match args.value_of("output").unwrap() {
		"json" => match serde_json::to_writer(stdout(), &data) {
			Err(e) => {
				return Err(ZeusError::new(
					"aur",
					format!("Error serializing JSON: {}", e),
				))
			}
			_ => {}
		},
		_ => {
			if args.is_present("info") {
				for package in &data.results {
					print_pretty_package(package);
				}
			} else {
				for package in &data.results {
					logger.v(
						Level::Info,
						"aur",
						format!(
							"{} - {}\n\t{}",
							package.Name, package.Version, package.Description,
						),
					);
				}
			}
		}
	}

	Ok(())
}

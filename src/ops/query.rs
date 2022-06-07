use crate::aur;
use crate::config;
use crate::error::{zerr, Result, ZeusError};
use crate::log::{self, Level};

use clap::ArgMatches;

use std::io::stdout;

macro_rules! print_if_some {
	($a:expr,$b:expr) => {{
		match $b {
			None => {},
			Some(v) => {
				println!("{0: <16}: {1}", $a, v);
			},
		}
	}};
}

macro_rules! print_vec_if_some {
	($a:expr,$b:expr) => {{
		match $b {
			None => {},
			Some(v) => {
				println!("{0: <16}: {1}", $a, v.join(" "));
			},
		}
	}};
}

fn print_pretty_package(package: &aur::Package) {
	print_if_some!("Name", &package.Name);
	print_if_some!("Version", &package.Version);
	print_if_some!("Description", &package.Description);
	print_if_some!("URL", &package.URL);

	print_vec_if_some!("License", &package.License);
	print_vec_if_some!("Groups", &package.Groups);
	print_vec_if_some!("Provides", &package.Provides);
	print_vec_if_some!("Depends On", &package.Depends);
	print_vec_if_some!("Optional Deps", &package.OptDepends);
	print_vec_if_some!("Conflicts", &package.Conflicts);
	print_vec_if_some!("Replaces", &package.Replaces);

	println!(
		"{0: <16}: {1}",
		"Maintainer",
		&package.Maintainer.as_ref().unwrap_or(&"none".to_owned())
	);

	print_if_some!("Last Modified", &package.LastModified);
	print_if_some!("First Submitted", &package.FirstSubmitted);

	println!(
		"{0: <16}: {1}",
		"Out of date",
		&package.OutOfDate.unwrap_or(0)
	);

	print_if_some!("Popularity", &package.Popularity);
	print_if_some!("Votes", &package.NumVotes);
}

pub async fn query(
	logger: &mut log::Logger,
	cfg: &mut config::AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	if cfg.keywords.is_empty() {
		return Err(ZeusError::new("No keywords specified"));
	}

	let by = args.value_of_t::<aur::By>("by").unwrap();

	let res = match args.is_present("info") {
		true => cfg.aur.info(&cfg.keywords).await,
		false => cfg.aur.search(by, &cfg.keywords).await,
	};

	let data = zerr!(res, "Error: ");

	match args.value_of("output").unwrap() {
		"json" => zerr!(
			serde_json::to_writer(stdout(), &data),
			"Error serializing JSON: "
		),
		_ => {
			if args.is_present("info") {
				for package in &data.results {
					print_pretty_package(package);
				}
			} else {
				for package in &data.results {
					logger.v(
						Level::Info,
						format!(
							"{} - {}\n\t{}",
							package
								.Name
								.as_ref()
								.unwrap_or(&"unnamed".to_owned()),
							package
								.Version
								.as_ref()
								.unwrap_or(&"0.0.0-0".to_owned()),
							package.Description.as_ref().unwrap_or(
								&"No description".to_owned()
							),
						),
					);
				}
			}
		},
	}

	Ok(())
}

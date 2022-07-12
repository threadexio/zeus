use std::io::stdout;

use super::prelude::*;
use crate::aur;

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

pub fn query(
	_term: &mut Terminal,
	mut cfg: AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	cfg.keywords = args
		.values_of("keywords")
		.unwrap_or_default()
		.map(|x| x.to_owned())
		.collect();

	if cfg.keywords.is_empty() {
		return Err(ZeusError::new(
			"zeus".to_owned(),
			"No keywords specified".to_owned(),
		));
	}

	let by = args.value_of_t::<aur::By>("by").unwrap();

	let res = match args.is_present("info") {
		true => cfg.aur.info(&cfg.keywords),
		false => cfg.aur.search(by, &cfg.keywords),
	};

	let data = zerr!(res, "aur", "Error: ");

	match args.value_of("output").unwrap() {
		"json" => zerr!(
			serde_json::to_writer(stdout(), &data.results),
			"zeus",
			"Cannot serialize JSON: "
		),
		_ => {
			if args.is_present("info") {
				for package in &data.results {
					print_pretty_package(package);
				}
			} else {
				for package in &data.results {
					println!(
						"{} {} - {}\n    {}",
						"=>".green(),
						package
							.Name
							.as_ref()
							.unwrap_or(&"".to_owned())
							.bold(),
						package
							.Version
							.as_ref()
							.unwrap_or(&"".to_owned())
							.bright_blue(),
						package
							.Description
							.as_ref()
							.unwrap_or(&"".to_owned()),
					);
				}
			}
		},
	}

	Ok(())
}

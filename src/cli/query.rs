use super::prelude::*;

macro_rules! print_info {
	($a:expr, $b:expr) => {
		println!("{0: <16}: {1}", $a, $b);
	};
}

macro_rules! print_if_some {
	($a:expr,$b:expr) => {{
		match $b {
			None => {},
			Some(v) => {
				print_info!($a, v);
			},
		}
	}};
}

macro_rules! print_vec {
	($a:expr,$b:expr) => {{
		match $b {
			Some(v) => {
				if !v.is_empty() {
					print_info!($a, v.join(" "));
				}
			},
			None => {},
		}
	}};
}

fn print_pretty_package(package: &aur::Package) {
	print_info!("Name", &package.name);
	print_info!("Version", &package.version);
	print_if_some!("Description", &package.description);
	print_info!("Last Modified", &package.last_modified);
	print_info!("First Submitted", &package.first_submitted);
	print_info!("Popularity", &package.popularity);
	print_info!("Votes", &package.num_votes);

	print_vec!("License", &package.license);
	print_vec!("Groups", &package.groups);
	print_vec!("Provides", &package.provides);
	print_vec!("Depends On", &package.depends);
	print_vec!("Optional Deps", &package.opt_depends);
	print_vec!("Conflicts", &package.conflicts);
	print_vec!("Replaces", &package.replaces);

	print_if_some!("URL", &package.url);
	print_if_some!("Maintainer", &package.maintainer);
	print_if_some!("Out Of Date", &package.out_of_date);

	println!();
}

pub fn query(
	db: &mut db::Db,
	aur: &mut aur::Aur,
	opts: QueryOptions,
) -> Result<()> {
	if opts.keywords.is_empty() {
		let pkgs = db
			.list_pkgs()
			.context("Unable to get synced packages")?;

		match opts.output {
			Output::Pretty => {
				for x in pkgs {
					println!("{}", x.name());
				}
			},
			Output::Json => {
				println!(
					"{}",
					serde_json::to_string(
						&pkgs
							.iter()
							.map(|x| x.name())
							.collect::<Vec<_>>()
					)
					.context(
						"Unable to serialize packages to JSON"
					)?
				);
			},
		}

		return Ok(());
	}

	debug!("Initializing AUR client");

	debug!("Requesting packages");

	let packages = match opts.info {
		true => aur.info(opts.keywords.iter()),
		false => aur.search(opts.by, opts.keywords.iter()),
	}
	.context("Unable to request packages from AUR")?;

	match opts.output {
		Output::Json => {
			serde_json::to_writer(std::io::stdout(), &packages)
				.context("Cannot serialize JSON")?
		},
		Output::Pretty => {
			if opts.info {
				for package in &packages {
					print_pretty_package(package);
				}
			} else {
				for package in &packages {
					info!(
						"{} - {}{}",
						package.name.bold(),
						package.version.bright_blue(),
						match package.description {
							Some(ref desc) =>
								format!("\n      {}", desc),
							None => "".to_string(),
						}
					);
				}
			}
		},
	}

	Ok(())
}
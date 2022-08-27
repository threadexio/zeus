use super::prelude::*;
use crate::aur;

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

macro_rules! print_vec_if_nonempty {
	($a:expr,$b:expr) => {{
		if !$b.is_empty() {
			print_info!($a, $b.join(" "));
		}
	}};
}

fn print_pretty_package(package: &aur::Package) {
	print_info!("Name", &package.name);
	print_info!("Version", &package.version);
	print_info!("Description", &package.description);
	print_info!("URL", &package.url);
	print_info!("Last Modified", &package.last_modified);
	print_info!("First Submitted", &package.first_submitted);
	print_info!("Popularity", &package.popularity);
	print_info!("Votes", &package.num_votes);

	print_vec_if_nonempty!("License", &package.license);
	print_vec_if_nonempty!("Groups", &package.groups);
	print_vec_if_nonempty!("Provides", &package.provides);
	print_vec_if_nonempty!("Depends On", &package.depends);
	print_vec_if_nonempty!("Optional Deps", &package.opt_depends);
	print_vec_if_nonempty!("Conflicts", &package.conflicts);
	print_vec_if_nonempty!("Replaces", &package.replaces);

	print_if_some!("Maintainer", &package.maintainer);
	print_if_some!("Out Of Date", &package.out_of_date);
}

pub fn query(
	_term: &mut Terminal,
	cfg: Config,
	opts: &mut QueryOptions,
) -> Result<()> {
	if opts.keywords.is_empty() {
		return Err(other!("No keywords specified"));
	}

	let res = match opts.info {
		true => cfg.aur.info(opts.keywords.iter()),
		false => {
			cfg.aur.search(opts.by.clone(), opts.keywords.iter())
		},
	};

	let data = err!(res, "Error");

	debug!("Raw response: {:?}", data);

	use aur::Output;
	match opts.output {
		Output::Json => err!(
			serde_json::to_writer(std::io::stdout(), &data.results),
			"Cannot serialize JSON"
		),
		Output::Pretty => {
			if opts.info {
				for package in &data.results {
					print_pretty_package(package);
				}
			} else {
				for package in &data.results {
					println!(
						"{} {} - {}\n    {}",
						"=>".green(),
						package.name.bold(),
						package.version.bright_blue(),
						package.description
					);
				}
			}
		},
	}

	Ok(())
}

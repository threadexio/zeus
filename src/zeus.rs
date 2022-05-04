mod aur;
mod cli;
mod config;
mod error;
mod log;
mod ops;
mod util;

use aur::Aur;
use log::Level;
use util::Lockfile;

use bollard::Docker;

use std::env::remove_var;
use std::fs::read_dir;
use std::path::Path;
use std::process::exit;

#[tokio::main]
async fn main() {
	let args = cli::build().get_matches();

	let mut logger = log::Logger::new(
		log::Stream::Stdout,
		match args.value_of("color") {
			Some("always") => log::ColorChoice::Always,
			Some("never") => log::ColorChoice::Never,
			_ => log::ColorChoice::Auto,
		},
	);

	logger.verbose = args.is_present("verbose");

	#[cfg(debug_assertions)]
	logger.v(Level::Debug, "docker", "Connecting...");

	let mut cfg = config::AppConfig {
		verbose: args.is_present("verbose"),
		force: args.is_present("force"),

		// this should never fail, we set the default value in cli.rs
		builddir: args.value_of("builddir").unwrap().to_owned(),

		aur: Aur::new()
			.host(args.value_of("aur").unwrap().to_owned())
			.build(),

		// initialization of the rest will be in the code that handles the subcommands
		..Default::default()
	};

	remove_var("DOCKER_HOST"); // just to make sure
	let docker = match Docker::connect_with_local_defaults() {
		Ok(v) => v,
		Err(e) => {
			logger.v(
				Level::Error,
				"docker",
				format!("Unable to connect to daemon: {}", e),
			);
			exit(1);
		}
	};

	#[cfg(debug_assertions)]
	logger.v(Level::Debug, "filesystem", "Creating lockfile...");

	let lockfile = match Lockfile::new(Path::new(&format!("{}/zeus.lock", &cfg.builddir))) {
		Ok(v) => v,
		Err(e) => {
			logger.v(
				Level::Error,
				"filesystem",
				format!("Cannot create lock: {}", e),
			);
			exit(1);
		}
	};

	let res = match args.subcommand() {
		Some(("sync", sync_args)) => {
			cfg.upgrade = sync_args.is_present("upgrade");
			cfg.buildargs = sync_args
				.value_of("buildargs")
				.unwrap_or("")
				.split_ascii_whitespace()
				.map(|x| x.to_owned())
				.collect();

			cfg.image = sync_args.value_of("image").unwrap().to_owned();
			cfg.name = sync_args.value_of("name").unwrap().to_owned();

			// Yeah, i will be able to understand this code 6 months later
			//
			//								- The clown who wrote this
			cfg.packages = sync_args
				.values_of("packages")
				.map(|x| x.map(|y| y.to_owned()).collect::<Vec<String>>())
				.unwrap_or_default();

			// We need to mimic `pacman -Su` which upgrades everything
			// this is what i like to call nested hell
			if cfg.upgrade && cfg.packages.is_empty() {
				match read_dir(&cfg.builddir) {
					Err(e) => {
						logger.v(
							Level::Error,
							"filesystem",
							format!("Cannot list directory: {}", e),
						);
						exit(1);
					}
					Ok(v) => {
						for r in v {
							match r {
								Err(e) => {
									logger.v(
										Level::Warn,
										"filesystem",
										format!("Cannot read package directory: {}", e),
									);
								}
								Ok(entry) => {
									if entry.path().is_dir() {
										match entry.file_name().into_string() {
											Err(e) => logger.v(
												Level::Warn,
												"filesystem",
												format!("Found invalid package: {:?}", e),
											),
											Ok(name) => cfg.packages.push(name),
										}
									}
								}
							}
						}
					}
				}
			} else if cfg.packages.is_empty() {
				logger.v(
					Level::Error,
					config::PROGRAM_NAME,
					"No packages specified. See --help!",
				);
				exit(1);
			}

			#[cfg(debug_assertions)]
			logger.v(Level::Debug, config::PROGRAM_NAME, format!("{:?}", cfg));

			#[cfg(debug_assertions)]
			logger.v(Level::Debug, "filesystem", "Obtaining lock...");

			match lockfile.lock() {
				Ok(_) => {}
				Err(e) => {
					logger.v(
						Level::Error,
						"filesystem",
						format!("Cannot obtain lock: {}", e),
					);
					exit(1);
				}
			};

			ops::sync(&mut logger, docker, cfg).await
		}
		Some(("build", build_args)) => {
			cfg.archive = build_args.value_of("archive").unwrap().to_owned();
			cfg.dockerfile = build_args.value_of("dockerfile").unwrap().to_owned();

			cfg.image = build_args.value_of("image").unwrap().to_owned();
			cfg.name = build_args.value_of("name").unwrap().to_owned();

			#[cfg(debug_assertions)]
			logger.v(Level::Debug, config::PROGRAM_NAME, format!("{:?}", cfg));

			#[cfg(debug_assertions)]
			logger.v(Level::Debug, "filesystem", "Obtaining lock...");

			match lockfile.lock() {
				Ok(_) => {}
				Err(e) => {
					logger.v(
						Level::Error,
						"filesystem",
						format!("Cannot obtain lock: {}", e),
					);
					exit(1);
				}
			};

			ops::build(&mut logger, docker, cfg).await
		}
		Some(("query", query_args)) => {
			cfg.keywords = query_args
				.values_of("keywords")
				.map(|x| x.map(|y| y.to_owned()).collect::<Vec<String>>())
				.unwrap_or_default();

			ops::query(&mut logger, cfg, query_args).await
		}
		Some(("misc", misc_args)) => ops::misc(misc_args),
		_ => {
			#[cfg(debug_assertions)]
			logger.v(
				Level::Debug,
				config::PROGRAM_NAME,
				"Subcommand given didn't match anything. Check the code!",
			);

			exit(-1);
		}
	};

	// we need to await in the respective call because otherwise we might release the lock before finishing, remember this code runs asynchronously
	match res {
		Ok(_) => exit(0),
		Err(e) => {
			logger.v(Level::Error, e.facility, e.data);
			exit(1);
		}
	}
}

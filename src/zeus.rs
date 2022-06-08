mod aur;
mod cli;
mod config;
mod error;
mod log;
mod ops;
mod util;

use aur::Aur;

use bollard::Docker;

use std::env::remove_var;
use std::path::Path;
use std::process::exit;

#[tokio::main]
async fn main() {
	remove_var("DOCKER_HOST"); // just to make sure

	let args = cli::build().get_matches();

	let mut logger = log::Logger {
		debug: args.is_present("debug"),
		out: log::Stream::Stderr,
		..Default::default()
	};

	match args.value_of("color") {
		Some("always") => {
			log::control::set_override(true);
		},
		Some("never") => {
			log::control::set_override(false);
		},
		_ => {},
	}

	let mut cfg = config::AppConfig {
		debug: logger.debug,
		force: args.is_present("force"),

		// this should never fail, we set the default value in cli.rs
		builddir: args.value_of("builddir").unwrap().to_owned(),

		aur: Aur::new()
			.host(args.value_of("aur").unwrap().to_owned())
			.build(),

		// initialization of the rest will be in the code that handles the subcommands
		..Default::default()
	};

	if cfg.force {
		cfg.buildargs.push("--force".to_owned());
	}

	/* let res = match args.subcommand() {
		Some(("sync", sync_args)) => {
			cfg.upgrade = sync_args.is_present("upgrade");

			if let Some(value) = sync_args.value_of("buildargs") {
				cfg.buildargs = value
					.split_ascii_whitespace()
					.map(|x| x.to_owned())
					.collect();
			}

			cfg.image =
				sync_args.value_of("image").unwrap().to_owned();
			cfg.name = sync_args.value_of("name").unwrap().to_owned();

			if let Some(values) = sync_args.values_of("packages") {
				for pkg in values {
					cfg.packages.push(pkg.to_owned());
				}
			}

			// We need to mimic `pacman -Su` which upgrades everything
			// this is what i like to call nested hell
			if cfg.upgrade && cfg.packages.is_empty() {
				match read_dir(&cfg.builddir) {
					Err(e) => {
						logger.v(
							Level::Error,
							format!(
								"Cannot list build directory: {}",
								e
							),
						);
						exit(1);
					},
					Ok(v) => {
						for r in v {
							match r {
								Err(e) => {
									logger.v(
										Level::Warn,
										format!("Cannot read package directory: {}", e),
									);
								},
								Ok(entry) => {
									if entry.path().is_dir() {
										match entry.file_name().into_string() {
											Err(e) => logger.v(
												Level::Warn,
												format!("Found invalid package: {:?}", e),
											),
											Ok(name) => cfg.packages.push(name),
										}
									}
								},
							}
						}
					},
				}
			} else if cfg.packages.is_empty() {
				logger.v(
					Level::Error,
					"No packages specified. See --help!",
				);
				exit(1);
			}

			#[cfg(debug_assertions)]
			logger.v(Level::Debug, format!("{:?}", cfg));

			#[cfg(debug_assertions)]
			logger.v(Level::Debug, "Obtaining lock...");

			match lockfile.lock() {
				Ok(_) => {},
				Err(e) => {
					logger.v(
						Level::Error,
						format!("Cannot obtain lock: {}", e),
					);
					exit(1);
				},
			};

			ops::sync(&mut logger, docker, cfg).await
		},
		Some(("remove", remove_args)) => {
			if let Some(values) = remove_args.values_of("packages") {
				for pkg in values {
					cfg.packages.push(pkg.to_owned());
				}
			}

			if cfg.packages.is_empty() {
				logger.v(
					Level::Error,
					"No packages specified. See --help!",
				);
				exit(1);
			}

			#[cfg(debug_assertions)]
			logger.v(Level::Debug, format!("{:?}", cfg));

			#[cfg(debug_assertions)]
			logger.v(Level::Debug, "Obtaining lock...");

			match lockfile.lock() {
				Ok(_) => {},
				Err(e) => {
					logger.v(
						Level::Error,
						format!("Cannot obtain lock: {}", e),
					);
					exit(1);
				},
			};

			ops::remove(&mut logger, cfg)
		},
		Some(("build", build_args)) => {
			cfg.archive =
				build_args.value_of("archive").unwrap().to_owned();
			cfg.dockerfile =
				build_args.value_of("dockerfile").unwrap().to_owned();

			cfg.image =
				build_args.value_of("image").unwrap().to_owned();
			cfg.name =
				build_args.value_of("name").unwrap().to_owned();

			#[cfg(debug_assertions)]
			logger.v(Level::Debug, format!("{:?}", cfg));

			#[cfg(debug_assertions)]
			logger.v(Level::Debug, "Obtaining lock...");

			match lockfile.lock() {
				Ok(_) => {},
				Err(e) => {
					logger.v(
						Level::Error,
						format!("Cannot obtain lock: {}", e),
					);
					exit(1);
				},
			};

			ops::build(&mut logger, docker, cfg).await
		},
		Some(("query", query_args)) => {
			if let Some(values) = query_args.values_of("keywords") {
				for keyword in values {
					cfg.packages.push(keyword.to_owned());
				}
			}

			ops::query(&mut logger, cfg, query_args).await
		},
		Some(("misc", misc_args)) => ops::misc(misc_args),
		_ => {
			#[cfg(debug_assertions)]
			logger.v(
				Level::Debug,
				"Subcommand given didn't match anything. Check the code!",
			);

			exit(-1);
		},
	}; */

	let (command_name, command_args) = args.subcommand().unwrap();

	let res = ops::run_operation(
		command_name,
		&mut logger,
		&mut cfg,
		command_args,
	)
	.await;

	match res {
		Ok(_) => exit(0),
		Err(e) => {
			logger.e(e.caller, e.message);
			exit(1);
		},
	}
}

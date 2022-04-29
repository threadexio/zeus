use crate::config::{PROGRAM_DESC, PROGRAM_NAME, PROGRAM_VERSION};

use std::io::Write;

use clap::{Arg, Command};

use clap_complete::generate;
pub use clap_complete::Shell;

use const_format::formatcp;
use default_env::default_env;

pub fn build_subcommands() -> Vec<Command<'static>> {
	vec![
		////////////////////////////////////////////////////
		Command::new("sync")
			.short_flag('S')
			.about("Sync packages")
			.arg(
				Arg::new("upgrade")
					.short('u')
					.long("upgrade")
					.help("Upgrade packages")
					.takes_value(false)
			)
			.arg(
				Arg::new("buildargs")
					.long("buildargs")
					.help("Extra arguments for makepkg")
					.takes_value(true)
			)
			.arg(
				Arg::new("image")
					.long("image")
					.help("Builder image name")
					.default_value(default_env!("DEFAULT_IMAGE", "zeus-builder"))
			)
			.arg(
				Arg::new("name")
					.long("name")
					.help("Builder container name")
					.default_value(default_env!("DEFAULT_NAME", "zeus-builder"))
			)
			.arg(
				Arg::new("packages")
					.help("Package names")
					.multiple_occurrences(true)
			),
		////////////////////////////////////////////////////
		Command::new("build")
			.short_flag('B')
			.about("Build/Update builder image")
			.arg(
				Arg::new("archive")
					.long("archive")
					.help("Builder image archive")
					.default_value(default_env!(
						"DEFAULT_ARCHIVE",
						"/usr/local/share/zeus/builder.tar.gz"
					))
			)
			.arg(
				Arg::new("dockerfile")
					.long("dockerfile")
					.help("Builder image dockerfile in archive")
					.default_value(default_env!("DEFAULT_DOCKERFILE", "Dockerfile"))
			)
			.arg(
				Arg::new("image")
					.long("image")
					.help("Builder image name")
					.default_value(default_env!("DEFAULT_IMAGE", "zeus-builder"))
			)
			.arg(
				Arg::new("name")
					.long("name")
					.help("Builder container name")
					.default_value(default_env!("DEFAULT_NAME", "zeus-builder"))
			),
		////////////////////////////////////////////////////
		Command::new("misc")
			.about("Generate shell completions & others")
			.arg(
				Arg::new("shell")
					.long("shell")
					.help("Specify shell to generate completions for")
					.possible_values(Shell::possible_values())
			),
		////////////////////////////////////////////////////
	]
}

pub fn build() -> Command<'static> {
	Command::new(PROGRAM_NAME)
		.version(PROGRAM_VERSION)
		.about(PROGRAM_DESC)
		.long_version(formatcp!(
			r#"{}

	 _oo     
  >-(_  \    Copyright lololol (C) 2022 1337 threadexio
	/ _/     
   / /       This program may be freely distributed under
  / (        the terms of the GNU General Public License v3.0.
 (   `-.     
  `--.._)    
"#,
			PROGRAM_VERSION
		))
		.arg(
			Arg::new("color")
				.long("color")
				.help("Colorize the output")
				.possible_value("auto")
				.possible_value("always")
				.possible_value("never")
				.default_value("auto")
		)
		.arg(
			Arg::new("verbose")
				.short('v')
				.long("verbose")
				.help("Be verbose")
				.takes_value(false)
		)
		.arg(
			Arg::new("force")
				.long("force")
				.help("Ignore all warnings")
				.takes_value(false)
		)
		.arg(
			Arg::new("builddir")
				.long("builddir")
				.help("Package build directory")
				.default_value(default_env!("DEFAULT_BUILDDIR", "/var/cache/aur"))
		)
		.subcommand_required(true)
		.subcommands(build_subcommands())
}

pub fn make_completions(s: Shell, buf: &mut dyn Write) {
	generate(s, &mut build(), PROGRAM_NAME, buf);
}

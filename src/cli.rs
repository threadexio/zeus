use crate::config::{self, defaults};

use clap::{Arg, Command};

use std::io::Write;

use clap_complete::generate;
pub use clap_complete::Shell;

use const_format::formatcp;

pub fn build_subcommands() -> Vec<Command<'static>> {
	vec![
		////////////////////////////////////////////////////
		Command::new("sync")
			.short_flag('S')
			.long_flag("sync")
			.about("Sync packages")
			.arg(
				Arg::new("upgrade")
					.short('u')
					.long("upgrade")
					.help("Upgrade packages")
					.takes_value(false),
			)
			.arg(
				Arg::new("buildargs")
					.long("buildargs")
					.help("Extra arguments for makepkg")
					.takes_value(true),
			)
			.arg(
				Arg::new("name")
					.long("name")
					.help("Builder machine name")
					.default_value(defaults::BUILDER_NAME),
			)
			.arg(
				Arg::new("packages")
					.help("Packages to sync")
					.multiple_occurrences(true),
			),
		////////////////////////////////////////////////////
		Command::new("remove")
			.short_flag('R')
			.long_flag("remove")
			.about("Remove packages")
			.arg(
				Arg::new("name")
					.long("name")
					.help("Builder machine name")
					.default_value(defaults::BUILDER_NAME),
			)
			.arg(
				Arg::new("packages")
					.help("Packages to remove")
					.multiple_occurrences(true),
			),
		////////////////////////////////////////////////////
		Command::new("build")
			.short_flag('B')
			.long_flag("build")
			.about("Build/Update builder image")
			.arg(
				Arg::new("image")
					.long("image")
					.help("Builder image name")
					.default_value(defaults::BUILDER_IMAGE),
			)
			.arg(
				Arg::new("name")
					.long("name")
					.help("Builder machine name")
					.default_value(defaults::BUILDER_NAME),
			),
		////////////////////////////////////////////////////
		Command::new("query")
			.short_flag('Q')
			.long_flag("query")
			.about("Query the AUR")
			.arg(
				Arg::new("info")
					.short('i')
					.long("info")
					.help("Display additional information on results")
					.takes_value(false)
					.conflicts_with("by"),
			)
			.arg(
				Arg::new("by")
					.long("by")
					.help("Query AUR packages by")
					.possible_values([
						"name",
						"description",
						"maintainer",
						"depends",
						"makedepends",
						"optdepends",
						"checkdepends",
					])
					.default_value("description")
					.conflicts_with("info"),
			)
			.arg(
				Arg::new("output")
					.long("output")
					.help("Output format")
					.possible_values(["pretty", "json"])
					.default_value("pretty"),
			)
			.arg(
				Arg::new("keywords")
					.help("Keywords to use")
					.multiple_occurrences(true),
			),
		////////////////////////////////////////////////////
		Command::new("completions")
			.long_flag("completions")
			.about("Generate shell completions & others")
			.arg(
				Arg::new("shell")
					.long("shell")
					.help("Specify shell to generate completions for")
					.possible_values(Shell::possible_values()),
			),
		////////////////////////////////////////////////////
		Command::new("runtime")
			.long_flag("runtime")
			.about("Various runtime operations")
			.arg(
				Arg::new("list")
					.short('l')
					.long("list")
					.help("List all available runtimes")
					.takes_value(false)
					.exclusive(true),
			),
		////////////////////////////////////////////////////
	]
}

pub fn build() -> Command<'static> {
	Command::new(config::NAME)
		.version(config::VERSION)
		.about(config::DESCRIPTION)
		.long_version(formatcp!(
			r#"{}

     _oo     Copyright lololol (C) 2022 {}
  >-(_  \
    / _/     This program may be freely distributed under
   / /       the terms of the GNU General Public License v3.0.
  / (
 (   `-.     {}
  `--.._)
             Defaults:
               DATA_DIR      | {}
               BUILDER_NAME  | {}
               BUILDER_IMAGE | {}
               BUILD_DIR     | {}
               AUR_HOST      | {}
               RUNTIME       | {}
               RUNTIME_DIR   | {}
"#,
			config::VERSION,
			config::AUTHORS,
			config::HOMEPAGE,
			defaults::DATA_DIR,
			defaults::BUILDER_NAME,
			defaults::BUILDER_IMAGE,
			defaults::BUILD_DIR,
			defaults::AUR_HOST,
			defaults::RUNTIME,
			defaults::RUNTIME_DIR,
		))
		.arg(
			Arg::new("color")
				.long("color")
				.help("Colorize the output")
				.possible_value("auto")
				.possible_value("always")
				.possible_value("never")
				.default_value("auto"),
		)
		.arg(
			Arg::new("debug")
				.short('d')
				.long("debug")
				.help("Show debug logs")
				.takes_value(false),
		)
		.arg(
			Arg::new("force")
				.long("force")
				.help("Ignore all warnings")
				.takes_value(false),
		)
		.arg(
			Arg::new("builddir")
				.long("builddir")
				.help("Package build directory")
				.default_value(defaults::BUILD_DIR),
		)
		.arg(
			Arg::new("aur")
				.long("aur")
				.help("AUR host")
				.default_value(defaults::AUR_HOST),
		)
		.arg(
			Arg::new("rt")
				.long("rt")
				.help("Specify runtime to use")
				.default_value(defaults::RUNTIME),
		)
		.arg(
			Arg::new("rtdir")
				.long("rtdir")
				.help("Specify directory to search for runtimes")
				.default_value(defaults::RUNTIME_DIR),
		)
		.subcommand_required(true)
		.subcommands(build_subcommands())
}

pub fn make_completions(s: Shell, buf: &mut dyn Write) {
	generate(s, &mut build(), config::NAME, buf);
}

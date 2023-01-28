#![allow(dead_code)]

use ::std::{
	path::{Path, PathBuf},
	str::FromStr,
};

use anyhow::Result;

use crate::{aur, constants, log};

mod misc;
pub use misc::*;

/// A macro to get a value from a config file.
///
/// # Example
/// ```rust,ignore
/// let config: toml::Value = toml::from_str(/* ... */).unwrap();
///
/// let value: Option<&str> = toml_path!(Some(&config), table1.table2.some_property);
/// ```
macro_rules! toml_path {
	($c:expr, $type:tt, $table:ident . $($tail:tt)*) => {
		toml_path!($c.and_then(|x| x.get(stringify!($table))).and_then(|x| x.as_table()), $type, $($tail)*)
	};
	($c:expr, array, $key:ident) => {
		$c.and_then(|x| x.get(stringify!($key))).and_then(|x| x.as_array())
	};
	($c:expr, str, $key:ident) => {
		$c.and_then(|x| x.get(stringify!($key))).and_then(|x| x.as_str())
	};
	($c:expr, bool, $key:ident) => {
		$c.and_then(|x| x.get(stringify!($key))).and_then(|x| x.as_bool())
	};
}

/// This macro is used to describe configuration options for zeus.
/// Either from the command line or a toml config file.
///
/// **Important**: This macro calls the `FromStr` implementation of each argument of type `str`!
///
/// # Example
/// ```rust,ignore
/// args! {
///     Opts {
/// //      argument                 default                       path inside         config
/// //      name        type         value                         config file         type (one of: array, bool, str)
///         test_arg:   String   =   "default".to_string();        args.test_arg   :   str,
///     }
/// }
/// ```
macro_rules! args {
	// Various type handlers
	(@matches, $self:expr, $matches:expr => $arg_name:ident : $arg_type:ty = $arg_default:expr; $($arg_path:ident).* : array) => {
		if let Some(v) = $matches.get_many::<String>(stringify!($arg_name)) {
			$self.$arg_name = v.map(|x| x.to_string()).collect();
		}
	};
	(@matches, $self:expr, $matches:expr => $arg_name:ident : $arg_type:ty = $arg_default:expr; $($arg_path:ident).* : str) => {
		if let Some(v) = $matches.get_one::<String>(stringify!($arg_name)) {
			if let Ok(v) = <$arg_type>::from_str(v) {
				$self.$arg_name = v;
			}
		}
	};
	(@matches, $self:expr, $matches:expr => $arg_name:ident : $arg_type:ty = $arg_default:expr; $($arg_path:ident).* : bool) => {
		if $matches.is_present(stringify!($arg_name)) {
			$self.$arg_name = true;
		};
	};

	(@file, $self:expr, $data:expr => $arg_name:ident : $arg_type:ty = $arg_default:expr; $($arg_path:ident).* : array) => {
		if let Some(v) = toml_path!(Some($data), array, $($arg_path).*) {
			$self.$arg_name = v.iter().filter_map(|x| x.as_str()).map(|x| x.to_string()).collect();
		}
	};
	(@file, $self:expr, $data:expr => $arg_name:ident : $arg_type:ty = $arg_default:expr; $($arg_path:ident).* : str) => {
		if let Some(v) = toml_path!(Some($data), str, $($arg_path).*) {
			if let Ok(v) = <$arg_type>::from_str(v) {
				$self.$arg_name = v;
			}
		}
	};
	(@file, $self:expr, $data:expr => $arg_name:ident : $arg_type:ty = $arg_default:expr; $($arg_path:ident).* : bool) => {
		if let Some(v) = toml_path!(Some($data), bool, $($arg_path).*) {
			$self.$arg_name = v;
		}
	};

	// Main
	($(#[$attr:meta])* $args_name:ident {
		$(
			$(#[$arg_attr:meta])*
			$arg_name:ident : $arg_type:ty = $arg_default:expr ; $($arg_path:ident).* : $arg_conf_type:ident,
		)*
	}) => {
		#[derive(Debug, Clone)]
		#[derive(serde::Serialize, serde::Deserialize)]
		$(#[$attr])*
		pub struct $args_name {
			$(
				$(#[$arg_attr])*
				pub $arg_name: $arg_type,
			)*
		}

		#[allow(dead_code)]
		impl $args_name {
			pub fn new(config: impl AsRef<str>, matches: &clap::ArgMatches) -> Result<Self> {
				let mut x = Self::default();
				x.parse_config(config.as_ref())?;
				x.parse_matches(matches);

				Ok(x)
			}

			/// Parse the options from `clap`'s `ArgMatches`.
			#[allow(unused)]
			pub fn parse_matches(&mut self, matches: &clap::ArgMatches) {
				$(
					args!(@matches, self, matches => $arg_name: $arg_type = $arg_default; $($arg_path).* : $arg_conf_type);
				)*
			}

			/// Parse the options from the config file at `file`.
			#[allow(unused)]
			pub fn parse_config(&mut self,config: &str) -> Result<()> {
				use toml::Value;
				let data: Value = toml::from_str(config)?;

				$(
					args!(@file, self, &data => $arg_name: $arg_type = $arg_default; $($arg_path).* : $arg_conf_type);
				)*

				Ok(())
			}
		}

		impl Default for $args_name {
			fn default() -> Self {
				Self {
					$(
						$arg_name: $arg_default,
					)*
				}
			}
		}
	};
}

args! {
	GlobalOptions {
		color:         Color        = Color::Always;                                 zeus.color     : str,
		log_level:     log::Level   = log::Level::Info;                              log.level      : str,
		build_dir:     PathBuf      = Path::new(constants::BUILD_DIR).to_path_buf(); zeus.build_dir : str,
		aur_url:       String       = constants::AUR_URL.to_string();                aur.url        : str,
		runtime:       String       = constants::RUNTIME.to_string();                zeus.runtime   : str,
		machine_name:  String       = constants::BUILDER_NAME.to_string();           machine.name   : str,
		machine_image: String       = constants::BUILDER_IMAGE.to_string();          machine.image  : str,
	}
}

args! {
	SyncOptions {
		upgrade:    bool        = false;  zeus.sync.upgrade      : bool,
		install:    bool        = false;  zeus.sync.install      : bool,
		build_args: Vec<String> = vec![]; zeus.sync.extra_args   : array,
		packages:   Vec<String> = vec![]; zeus.sync.__packages__ : array,
	}
}

args! {
	RemoveOptions {
		uninstall: bool        = false;  zeus.remove.uninstall    : bool,
		packages:  Vec<String> = vec![]; zeus.remove.__packages__ : array,
	}
}

args! {
	BuildOptions {}
}

args! {
	QueryOptions {
		info:     bool        = false;             zeus.query.__info__     : bool,
		by:       aur::By     = aur::By::NameDesc; zeus.query.by           : str,
		output:   Output      = Output::Pretty;    zeus.query.output       : str,
		keywords: Vec<String> = vec![];            zeus.query.__keywords__ : array,
	}
}

args! {
	CompletionOptions {
		shell: Shell = Shell::Bash; zeus.completions.shell : str,
	}
}

args! {
	RuntimeOptions {
		list: bool = false; zeus.runtime.__list__ : bool,
	}
}

/// Builds the clap command
pub fn app() -> clap::Command<'static> {
	use clap::{Arg, Command, ValueHint};

	let command = Command::new(constants::NAME)
		.about(constants::DESCRIPTION)
		.version(constants::VERSION)
		.long_version(constants::LONG_VERSION)
		.author(constants::AUTHORS)
		.subcommand_required(true)
		.arg_required_else_help(true)
		.arg(
			Arg::new("color")
				.long("color")
				.help("Colorize the output")
				.takes_value(true)
				.forbid_empty_values(true)
				.value_name("when")
				.value_parser(
					clap::builder::PossibleValuesParser::new(
						Color::possible_values(),
					),
				)
				.default_missing_value("always"),
		)
		.arg(
			Arg::new("log_level")
				.short('l')
				.long("level")
				.help("Set log level")
				.takes_value(true)
				.forbid_empty_values(true)
				.value_name("level")
				.value_parser(
					clap::builder::PossibleValuesParser::new(
						log::Level::possible_values(),
					),
				),
		)
		.arg(
			Arg::new("build_dir")
				.long("build-dir")
				.help("Package build directory")
				.takes_value(true)
				.forbid_empty_values(true)
				.value_name("dir")
				.value_hint(ValueHint::DirPath),
		)
		.arg(
			Arg::new("aur_url")
				.long("aur")
				.help("AUR URL")
				.takes_value(true)
				.forbid_empty_values(true)
				.value_name("url")
				.value_hint(ValueHint::Url),
		)
		.arg(
			Arg::new("runtime")
				.long("rt")
				.help("Specify runtime to use")
				.takes_value(true)
				.forbid_empty_values(true)
				.value_name("name")
				.value_hint(ValueHint::Other),
		)
		.arg(
			Arg::new("machine_name")
				.long("name")
				.help("Builder machine name")
				.takes_value(true)
				.forbid_empty_values(true)
				.value_name("name")
				.value_hint(ValueHint::Other),
		)
		.arg(
			Arg::new("machine_image")
				.long("image")
				.help("Builder machine image")
				.takes_value(true)
				.forbid_empty_values(true)
				.value_name("image")
				.value_hint(ValueHint::Other),
		);

	let command = command.subcommand(
		Command::new("sync")
			.short_flag('S')
			.about("Sync packages")
			.display_order(0)
			.arg(
				Arg::new("upgrade")
					.short('u')
					.long("upgrade")
					.help("Upgrade packages")
					.takes_value(false),
			)
			.arg(
				Arg::new("install")
					.long("install")
					.help("Install packages after build")
					.takes_value(false),
			)
			.arg(
				Arg::new("build_args")
					.long("build-args")
					.help("Extra arguments for makepkg")
					.takes_value(true)
					.forbid_empty_values(true)
					.value_name("args")
					.value_hint(ValueHint::Other)
					.multiple_values(true)
					.multiple_occurrences(true),
			)
			.arg(
				Arg::new("packages")
					.help("Packages to sync")
					.takes_value(true)
					.forbid_empty_values(true)
					.multiple_values(true)
					.multiple_occurrences(true),
			),
	);

	let command = command.subcommand(
		Command::new("remove")
			.short_flag('R')
			.about("Remove packages")
			.display_order(1)
			.arg(
				Arg::new("uninstall")
					.long("uninstall")
					.help("Uninstall packages after remove")
					.takes_value(false),
			)
			.arg(
				Arg::new("packages")
					.help("Package to remove")
					.takes_value(true)
					.forbid_empty_values(true)
					.multiple_values(true)
					.multiple_occurrences(true),
			),
	);

	let command = command.subcommand(
		Command::new("build")
			.short_flag('B')
			.about("Build/Update builder")
			.display_order(2),
	);

	let command = command.subcommand(
		Command::new("query")
			.short_flag('Q')
			.about("Query the AUR")
			.display_order(3)
			.arg(
				Arg::new("info")
					.short('i')
					.long("info")
					.help("Display additional information on results")
					.takes_value(false),
			)
			.arg(
				Arg::new("by")
					.short('b')
					.long("by")
					.help("Query AUR packages by")
					.takes_value(true)
					.forbid_empty_values(true)
					.value_name("rule")
					.value_parser(
						clap::builder::PossibleValuesParser::new(
							aur::By::possible_values(),
						),
					)
					.conflicts_with("info"),
			)
			.arg(
				Arg::new("output")
					.long("output")
					.help("Output format")
					.takes_value(true)
					.forbid_empty_values(true)
					.value_name("format")
					.value_parser(
						clap::builder::PossibleValuesParser::new(
							Output::possible_values(),
						),
					),
			)
			.arg(
				Arg::new("keywords")
					.takes_value(true)
					.forbid_empty_values(true)
					.multiple_values(true)
					.multiple_occurrences(true),
			),
	);

	let command = command.subcommand(
		Command::new("runtime")
			.about("Various runtime operations")
			.display_order(4)
			.arg(
				Arg::new("list")
					.short('l')
					.long("lost")
					.help("List all available runtimes")
					.takes_value(false),
			),
	);

	let command = command.subcommand(
		Command::new("completions")
			.about("Generate shell completions")
			.display_order(5)
			.arg(
				Arg::new("shell")
					.short('s')
					.long("shell")
					.help("Specify shell to generate completions for")
					.takes_value(true)
					.forbid_empty_values(true)
					.value_name("shell")
					.value_parser(
						clap::builder::PossibleValuesParser::new(
							Shell::possible_values(),
						),
					),
			),
	);

	command
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_clap_cli() {
		app().debug_assert()
	}
}

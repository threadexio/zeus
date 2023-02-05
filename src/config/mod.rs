use std::path::PathBuf;

use anyhow::{Context, Result};
use toml::Value;

mod macros;
pub mod types;

mod traits;
use traits::Config;

mod definition;
pub use definition::*;

use crate::log::macros::warning;

#[derive(Debug)]
pub enum Operation {
	Sync(SyncConfig),
	Remove(RemoveConfig),
	Build(BuildConfig),
	Query(QueryConfig),
	Runtime(RuntimeConfig),
	Completion(CompletionsConfig),
}

pub fn command() -> clap::Command {
	use crate::constants;
	GlobalConfig::command()
		.name(constants::NAME)
		.version(constants::VERSION)
		.long_version(constants::LONG_VERSION)
		.author(constants::AUTHORS)
		.subcommand_required(true)
		.disable_help_subcommand(true)
		.arg_required_else_help(true)
		.arg(
			clap::Arg::new("config_file")
				.long("config")
				.help("Set an alternate configuration file")
				.default_value(constants::CONFIG_FILE)
				.value_name("path")
				.value_hint(clap::ValueHint::FilePath)
				.value_parser(clap::value_parser!(PathBuf)),
		)
		.subcommand(SyncConfig::command())
		.subcommand(RemoveConfig::command())
		.subcommand(BuildConfig::command())
		.subcommand(QueryConfig::command())
		.subcommand(RuntimeConfig::command())
		.subcommand(CompletionsConfig::command())
}

#[derive(Debug)]
pub struct AppConfig {
	pub global: GlobalConfig,
	pub operation: Operation,
}

pub fn load() -> Result<AppConfig> {
	let matches = command().get_matches();

	let config_file = matches
		.get_one::<PathBuf>("config_file")
		.expect("missing default value for config_file");

	let try_load_config_file = || -> Result<Value> {
		let raw = std::fs::read_to_string(config_file).with_context(
			|| {
				format!(
					"Unable to read config file '{}'",
					config_file.display()
				)
			},
		)?;

		toml::from_str(&raw).with_context(|| {
			format!(
				"Unable to parse config file '{}'",
				config_file.display()
			)
		})
	};

	let config = match try_load_config_file() {
		Ok(v) => v,
		Err(e) => {
			warning!(
				"Unable to load config file '{}': {e}",
				config_file.display()
			);

			toml::from_str("")
				.expect("failed to create an empty toml::Value")
		},
	};

	Ok(AppConfig {
		global: GlobalConfig::new(&matches, &config)?,
		operation: match matches.subcommand() {
			Some(("sync", m)) => {
				Operation::Sync(SyncConfig::new(m, &config)?)
			},
			Some(("remove", m)) => {
				Operation::Remove(RemoveConfig::new(m, &config)?)
			},
			Some(("build", m)) => {
				Operation::Build(BuildConfig::new(m, &config)?)
			},
			Some(("query", m)) => {
				Operation::Query(QueryConfig::new(m, &config)?)
			},
			Some(("runtime", m)) => {
				Operation::Runtime(RuntimeConfig::new(m, &config)?)
			},
			Some(("completions", m)) => Operation::Completion(
				CompletionsConfig::new(m, &config)?,
			),
			_ => panic!("Invalid subcommand. This is a bug!"),
		},
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_clap_cli() {
		command().debug_assert();
	}
}

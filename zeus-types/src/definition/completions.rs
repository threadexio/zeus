use anyhow::Context;

use super::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionsConfig {
	pub shell: Shell,
}

impl Config for CompletionsConfig {
	fn command() -> Command {
		Command::new("completions")
			.about("Generate shell completions")
			.arg(
				Arg::new("shell")
					.short('s')
					.long("shell")
					.help("Specify shell to generate completions for")
					.value_name("shell")
					.value_parser(Shell::value_parser()),
			)
	}

	fn new(matches: &ArgMatches, config: &Value) -> Result<Self> {
		Ok(Self {
			shell: Shell::from_value(
				matches
					.get_one::<String>("shell")
					.map(|x| x.as_str())
					.or(
						config_path!(config => zeus.completions.Shell as str),
					)
					.unwrap_or("bash"),
			)
			.context("invalid value for 'zeus.completions.Shell'")?,
		})
	}
}

use zeus_aur::By;

use super::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryConfig {
	pub info: bool,
	pub by: By,
	pub output: Output,
	pub keywords: Vec<String>,
}

impl Config for QueryConfig {
	fn command() -> Command {
		Command::new("query")
			.short_flag('Q')
			.long_flag("query")
			.about("Query the AUR")
			.arg(
				Arg::new("info")
					.short('i')
					.long("info")
					.help("Display additional information on results")
					.action(ArgAction::SetTrue),
			)
			.arg(
				Arg::new("by")
					.short('b')
					.long("by")
					.help("Query AUR packages by")
					.value_name("rule")
					.value_parser(By::value_parser())
					.conflicts_with("info"),
			)
			.arg(
				Arg::new("output")
					.long("output")
					.help("Output format")
					.value_name("format")
					.value_parser(Output::value_parser()),
			)
			.arg(Arg::new("keywords").action(ArgAction::Append))
	}

	fn new(matches: &ArgMatches, config: &Value) -> Result<Self> {
		Ok(Self {
			info: matches.get_flag("info"),
			by: By::from_value(
				matches
					.get_one::<String>("by")
					.map(|x| x.as_str())
					.or(config_path!(config => zeus.query.By as str))
					.unwrap_or("namedesc"),
			)
			.context("invalid value for 'zeus.query.By'")?,
			output: Output::from_value(
				matches
					.get_one::<String>("output")
					.map(|x| x.as_str())
					.or(
						config_path!(config => zeus.query.Output as str),
					)
					.unwrap_or("pretty"),
			)
			.context("invalid value for 'zeus.query.Output'")?,
			keywords: matches
				.get_many::<String>("keywords")
				.map(|x| x.cloned().collect())
				.unwrap_or_default(),
		})
	}
}

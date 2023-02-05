use super::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
	pub list: bool,
}

impl Config for RuntimeConfig {
	fn command() -> Command {
		Command::new("runtime")
			.about("Various runtime operations")
			.arg(
				Arg::new("list")
					.short('l')
					.long("list")
					.help("List all available runtimes")
					.action(ArgAction::SetTrue)
					.exclusive(true),
			)
	}

	fn new(matches: &ArgMatches, _: &Value) -> Result<Self> {
		Ok(Self { list: matches.get_flag("list") })
	}
}

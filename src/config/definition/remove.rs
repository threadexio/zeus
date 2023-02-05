use super::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveConfig {
	pub uninstall: bool,
	pub packages: Vec<String>,
}

impl Config for RemoveConfig {
	fn command() -> Command {
		Command::new("remove")
			.short_flag('R')
			.long_flag("remove")
			.about("Remove packages")
			.arg(
				Arg::new("uninstall")
					.long("uninstall")
					.help("Uninstall packages after remove")
					.action(ArgAction::SetTrue),
			)
			.arg(
				Arg::new("packages")
					.help("Packages to remove")
					.action(ArgAction::Append),
			)
	}

	fn new(matches: &ArgMatches, config: &Value) -> Result<Self> {
		Ok(Self {
			uninstall: matches.get_flag("uninstall")
				| config_path!(config => zeus.remove.Uninstall as bool)
					.unwrap_or(false),
			packages: matches
				.get_many::<String>("packages")
				.map(|x| x.cloned().collect())
				.unwrap_or_default()
		})
	}
}

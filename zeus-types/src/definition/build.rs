use super::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {}

impl Config for BuildConfig {
	fn command() -> Command {
		Command::new("build")
			.short_flag('B')
			.long_flag("build")
			.about("Build/Update builder")
	}

	fn new(_: &ArgMatches, _: &Value) -> Result<Self> {
		Ok(Self {})
	}
}

use super::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
	pub upgrade: bool,
	pub install: bool,
	pub build_args: Vec<String>,
	pub packages: Vec<String>,
}

impl Config for SyncConfig {
	fn command() -> Command {
		Command::new("sync")
			.short_flag('S')
			.long_flag("sync")
			.about("Sync packages")
			.arg(
				Arg::new("upgrade")
					.short('u')
					.long("upgrade")
					.help("Upgrade packages")
					.action(ArgAction::SetTrue),
			)
			.arg(
				Arg::new("install")
					.long("install")
					.help("Install packages after build")
					.action(ArgAction::SetTrue),
			)
			.arg(
				Arg::new("build_args")
					.long("build-args")
					.help("Extra arguments for makepkg")
					.value_name("args")
					.value_hint(ValueHint::Other)
					.action(ArgAction::Append),
			)
			.arg(
				Arg::new("packages")
					.help("Packages to sync")
					.action(ArgAction::Append),
			)
	}

	fn new(matches: &ArgMatches, config: &Value) -> Result<Self> {
		Ok(Self {
			upgrade: matches.get_flag("upgrade")
				| config_path!(config => zeus.sync.Upgrade as bool)
					.unwrap_or(false),
			install: matches.get_flag("install")
				| config_path!(config => zeus.sync.Install as bool)
					.unwrap_or(false),
			build_args: {
				let x: Vec<_> = matches
					.get_many::<String>("build_args")
					.map(|x| x.cloned().collect())
					.unwrap_or_default();

				let mut y: Vec<_> = config_path!(config => zeus.sync.ExtraArgs as array<str>)
							.unwrap_or_default()
							.drain(..)
							.map(|x| x.to_string())
							.collect();

				y.extend(x);
				y
			},
			packages: matches
				.get_many::<String>("packages")
				.map(|x| x.cloned().collect())
				.unwrap_or_default(),
		})
	}
}

use std::collections::HashMap;

use anyhow::Context;

use super::prelude::*;
use crate::log;

pub type RuntimeOpts = HashMap<String, String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
	pub color: Color,
	pub log_level: log::Level,
	pub build_dir: PathBuf,
	pub aur_url: String,
	pub runtime: String,
	pub machine_name: String,
	pub machine_image: String,
	pub runtime_opts: RuntimeOpts,
}

impl Config for GlobalConfig {
	fn command() -> Command {
		Command::new("global")
			.arg(
				Arg::new("color")
					.long("color")
					.help("Colorize the output")
					.value_name("when")
					.value_parser(Color::value_parser()),
			)
			.arg(
				Arg::new("log_level")
					.short('l')
					.long("level")
					.help("Set log level")
					.value_name("level")
					.value_parser(log::Level::value_parser()),
			)
			.arg(
				Arg::new("build_dir")
					.long("build-dir")
					.help("Package build directory")
					.value_name("path")
					.value_hint(ValueHint::DirPath)
					.value_parser(value_parser!(String)),
			)
			.arg(
				Arg::new("aur_url")
					.long("aur")
					.help("AUR URL")
					.value_name("url")
					.value_hint(ValueHint::Url)
					.value_parser(value_parser!(String)),
			)
			.arg(
				Arg::new("runtime")
					.long("rt")
					.help("Specify runtime to use")
					.value_name("name")
					.value_hint(ValueHint::Other)
					.value_parser(value_parser!(String)),
			)
			.arg(
				Arg::new("machine_name")
					.long("name")
					.help("Builder machine name")
					.value_name("name")
					.value_hint(ValueHint::Other)
					.value_parser(value_parser!(String)),
			)
			.arg(
				Arg::new("machine_image")
					.long("image")
					.help("Builder machine image")
					.value_name("image")
					.value_hint(ValueHint::Other)
					.value_parser(value_parser!(String)),
			)
			.arg(
				Arg::new("runtime_opts")
					.long("rtcfg")
					.help("Set an option for the runtime")
					.value_name("key=value")
					.value_hint(ValueHint::Other)
					.action(ArgAction::Append),
			)
	}

	fn new(matches: &ArgMatches, config: &Value) -> Result<Self> {
		let color = Color::from_value(
			matches
				.get_one::<String>("color")
				.map(|x| x.as_str())
				.or(config_path!(config => zeus.Color as str))
				.unwrap_or("auto"),
		)
		.context("invalid value for 'zeus.Color'")?;

		let log_level = log::Level::from_value(
			matches
				.get_one::<String>("log_level")
				.map(|x| x.as_str())
				.or(config_path!(config => log.Level as str))
				.unwrap_or(constants::LOG_LEVEL),
		)
		.context("invalid value for 'log.Level'")?;

		let build_dir = Path::new(
			matches
				.get_one::<String>("build_dir")
				.map(|x| x.as_str())
				.or(config_path!(config => zeus.BuildDir as str))
				.unwrap_or(constants::BUILD_DIR),
		)
		.to_path_buf();

		let aur_url = matches
			.get_one::<String>("aur_url")
			.map(|x| x.as_str())
			.or(config_path!(config => aur.Url as str))
			.unwrap_or(constants::AUR_URL)
			.to_string();

		let runtime = matches
			.get_one::<String>("runtime")
			.map(|x| x.as_str())
			.or(config_path!(config => runtime.Name as str))
			.unwrap_or(constants::RUNTIME)
			.to_string();

		let machine_name = matches
			.get_one::<String>("machine_name")
			.map(|x| x.as_str())
			.or(config_path!(config => runtime.Builder as str))
			.unwrap_or(constants::BUILDER_NAME)
			.to_string();

		let machine_image = matches
			.get_one::<String>("machine_image")
			.map(|x| x.as_str())
			.or(config_path!(config => runtime.Image as str))
			.unwrap_or(constants::BUILDER_IMAGE)
			.to_string();

		let runtime_opts = {
			let mut opts = HashMap::new();

			if let Some(table) =
				config_path!(config => runtime as table)
					.and_then(|x| x.get(&runtime))
					.and_then(|x| x.as_table())
			{
				table
					.iter()
					.filter_map(|(k, v)| Some((k, v.as_str()?)))
					.map(|(k, v)| (k.to_string(), v.to_string()))
					.for_each(|(k, v)| {
						opts.insert(k, v);
					});
			}

			if let Some(values) =
				matches.get_many::<String>("runtime_opts")
			{
				values
					.filter_map(|x| x.split_once('='))
					.map(|(k, v)| (k.to_string(), v.to_string()))
					.for_each(|(k, v)| {
						opts.insert(k, v);
					});
			}

			opts
		};

		Ok(Self {
			color,
			log_level,
			build_dir,
			aur_url,
			runtime,
			machine_name,
			machine_image,
			runtime_opts,
		})
	}
}

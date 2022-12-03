pub mod macros;

pub use colored::Colorize;

#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	serde::Serialize,
	serde::Deserialize,
)]
pub enum Level {
	Fatal,
	Error,
	Warn,
	Info,
	Debug,
	Trace,
}

impl Level {
	pub fn possible_values() -> &'static [&'static str] {
		&["fatal", "error", "warn", "info", "debug", "trace"]
	}
}

impl std::str::FromStr for Level {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"fatal" => Ok(Self::Fatal),
			"error" => Ok(Self::Error),
			"warn" => Ok(Self::Warn),
			"info" => Ok(Self::Info),
			"debug" => Ok(Self::Debug),
			"trace" => Ok(Self::Trace),
			_ => Err(()),
		}
	}
}

#[allow(dead_code)]
pub fn set_color_enabled(enabled: bool) {
	colored::control::set_override(enabled)
}

#[doc(hidden)]
#[allow(dead_code)]
pub mod __private_log {
	use super::*;

	pub static mut MAX_LOG_LEVEL: Level = Level::Info;

	use colored::Colorize;
	use std::fmt::Display;

	pub fn imp_log<D: Display>(
		level: Level,
		facility: Option<&str>,
		message: D,
	) {
		unsafe {
			if level <= MAX_LOG_LEVEL {
				eprintln!(
					" {} {}{}",
					match level {
						Level::Trace => "==".bright_white().bold(),
						Level::Debug => "--".white(),
						Level::Info => "=>".green(),
						Level::Warn => "!!".yellow(),
						Level::Error => "**".red(),
						Level::Fatal => "**".bright_red().bold(),
					},
					match facility {
						Some(v) => format!("{}: ", v),
						None => "".to_string(),
					},
					message
				)
			}
		}
	}
}

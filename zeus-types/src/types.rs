use serde::{Deserialize, Serialize};

use crate::ConfigEnum;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Color {
	/// Always use colors
	Always,
	/// Never use colors
	Never,
	/// Automatically decide whether to use colors
	Auto,
}

impl ConfigEnum for Color {
	fn possible_values() -> &'static [&'static str] {
		&["never", "always", "auto"]
	}

	fn from_value(s: &str) -> Option<Self>
	where
		Self: Sized,
	{
		match s {
			"never" => Some(Self::Never),
			"always" => Some(Self::Always),
			"auto" => Some(Self::Auto),
			_ => None,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Output {
	/// Format the output for humans
	Pretty,
	/// Output JSON
	Json,
}

impl ConfigEnum for Output {
	fn possible_values() -> &'static [&'static str] {
		&["pretty", "json"]
	}

	fn from_value(s: &str) -> Option<Self>
	where
		Self: Sized,
	{
		match s {
			"pretty" => Some(Self::Pretty),
			"json" => Some(Self::Json),
			_ => None,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Shell {
	Bash,
	Fish,
	Zsh,
}

impl From<Shell> for clap_complete::Shell {
	fn from(s: Shell) -> Self {
		match s {
			Shell::Bash => Self::Bash,
			Shell::Fish => Self::Fish,
			Shell::Zsh => Self::Zsh,
		}
	}
}

impl ConfigEnum for Shell {
	fn possible_values() -> &'static [&'static str] {
		&["bash", "fish", "zsh"]
	}

	fn from_value(s: &str) -> Option<Self>
	where
		Self: Sized,
	{
		match s {
			"bash" => Some(Self::Bash),
			"fish" => Some(Self::Fish),
			"zsh" => Some(Self::Zsh),
			_ => None,
		}
	}
}

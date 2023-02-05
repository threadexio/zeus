use core::marker::Sized;

use anyhow::Result;
use clap::{builder::PossibleValuesParser, ArgMatches, Command};
use toml::Value;

use crate::{aur, log};

pub trait Config {
	fn command() -> Command;
	fn new(matches: &ArgMatches, config: &Value) -> Result<Self>
	where
		Self: Sized;
}

pub trait ConfigEnum {
	fn possible_values() -> &'static [&'static str];
	fn from_value(s: &str) -> Option<Self>
	where
		Self: Sized;

	fn value_parser() -> PossibleValuesParser {
		PossibleValuesParser::new(Self::possible_values())
	}
}

impl ConfigEnum for super::types::Color {
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

impl ConfigEnum for super::types::Output {
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

impl ConfigEnum for super::types::Shell {
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

impl ConfigEnum for log::Level {
	fn possible_values() -> &'static [&'static str] {
		&["fatal", "error", "warn", "info", "debug", "trace"]
	}

	fn from_value(s: &str) -> Option<Self>
	where
		Self: Sized,
	{
		match s {
			"fatal" => Some(Self::Fatal),
			"error" => Some(Self::Error),
			"warn" => Some(Self::Warn),
			"info" => Some(Self::Info),
			"debug" => Some(Self::Debug),
			"trace" => Some(Self::Trace),
			_ => None,
		}
	}
}

impl ConfigEnum for aur::By {
	fn possible_values() -> &'static [&'static str] {
		&[
			"name",
			"namedesc",
			"maintainer",
			"depends",
			"makedepends",
			"optdepends",
			"checkdepends",
		]
	}

	fn from_value(s: &str) -> Option<Self>
	where
		Self: Sized,
	{
		match s {
			"name" => Some(Self::Name),
			"namedesc" => Some(Self::NameDesc),
			"maintainer" => Some(Self::Maintainer),
			"depends" => Some(Self::Depends),
			"makedepends" => Some(Self::MakeDepends),
			"optdepends" => Some(Self::OptDepends),
			"checkdepends" => Some(Self::CheckDepends),
			_ => None,
		}
	}
}

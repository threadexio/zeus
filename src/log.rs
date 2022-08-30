use std::fmt::Display;

use colored::{Color, Colorize};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Serialize,
	Deserialize,
	ValueEnum,
)]
pub enum LogLevel {
	Error,
	Warn,
	Info,
	Debug,
}

impl From<LogLevel> for Color {
	fn from(v: LogLevel) -> Self {
		use LogLevel::*;
		match v {
			Error => Color::Red,
			Warn => Color::Yellow,
			Info => Color::Green,
			Debug => Color::Blue,
		}
	}
}

impl Default for LogLevel {
	fn default() -> Self {
		LogLevel::Info
	}
}

#[derive(Debug, Default)]
pub struct Logger {
	pub level: LogLevel,
}

#[allow(dead_code)]
impl Logger {
	pub(crate) fn log_impl(&self, level: LogLevel, data: String) {
		if level > self.level {
			return;
		}

		eprintln!(
			"{}{}{} {}",
			"[".bright_black(),
			level.to_string().color(level).bold(),
			"]".bright_black(),
			data.bright_white().bold()
		);
	}
}

pub static mut LOGGER: Logger = Logger { level: LogLevel::Info };

#[macro_export]
macro_rules! error {
		($($arg:tt)*) => ({
			#[allow(unused_unsafe)]
			unsafe {
				$crate::log::LOGGER.log_impl($crate::log::LogLevel::Error, format!($($arg)*))
			}
		});
	}
#[macro_export]
macro_rules! warn {
		($($arg:tt)*) => ({
			#[allow(unused_unsafe)]
			unsafe {
				$crate::log::LOGGER.log_impl($crate::log::LogLevel::Warn, format!($($arg)*))
			}
		});
	}
#[macro_export]
macro_rules! info {
		($($arg:tt)*) => ({
			#[allow(unused_unsafe)]
			unsafe {
				$crate::log::LOGGER.log_impl($crate::log::LogLevel::Info,format!($($arg)*))
			}
		});
	}
#[macro_export]
macro_rules! debug {
		($($arg:tt)*) => ({
			#[allow(unused_unsafe)]
			unsafe {
				$crate::log::LOGGER.log_impl($crate::log::LogLevel::Debug,format!($($arg)*))
			}
		});
	}

impl Display for LogLevel {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		use LogLevel::*;
		write!(
			f,
			"{}",
			match self {
				Error => "ERROR",
				Warn => "WARNING",
				Info => "INFO",
				Debug => "DEBUG",
			}
		)
	}
}

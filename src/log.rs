use std::fmt::Display;

use colored::{Color, Colorize};

#[derive(Debug, Clone, Copy)]
pub struct Colors {
	pub error: Color,
	pub warn: Color,
	pub info: Color,
	pub debug: Color,
}

#[allow(dead_code)]
impl Colors {
	pub fn new() -> Self {
		Self { ..Default::default() }
	}

	pub fn error(mut self, c: Color) -> Self {
		self.error = c;
		self
	}
	pub fn warn(mut self, c: Color) -> Self {
		self.warn = c;
		self
	}
	pub fn info(mut self, c: Color) -> Self {
		self.info = c;
		self
	}
	pub fn debug(mut self, c: Color) -> Self {
		self.debug = c;
		self
	}
}

impl Default for Colors {
	fn default() -> Self {
		Self {
			error: Color::Red,
			warn: Color::Yellow,
			info: Color::Green,
			debug: Color::Blue,
		}
	}
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Logger {
	pub colors: Colors,
	pub debug: bool,
}

#[allow(dead_code)]
impl Logger {
	fn log_impl(
		&self,
		level: &str,
		c: Color,
		caller: &str,
		data: &str,
	) {
		eprintln!(
			"{}{} {}{} {}",
			"[".bright_black(),
			level.color(c).bold(),
			caller.bold(),
			"]".bright_black(),
			data
		);
	}

	pub fn e<T>(&self, caller: &str, message: T)
	where
		T: Display,
	{
		self.log_impl(
			"ERROR",
			self.colors.error,
			caller,
			&message.to_string(),
		);
	}
	pub fn w<T>(&self, caller: &str, message: T)
	where
		T: Display,
	{
		self.log_impl(
			"WARN",
			self.colors.warn,
			caller,
			&message.to_string(),
		);
	}
	pub fn i<T>(&self, caller: &str, message: T)
	where
		T: Display,
	{
		self.log_impl(
			"INFO",
			self.colors.info,
			caller,
			&message.to_string(),
		);
	}
	pub fn d<T>(&self, caller: &str, message: T)
	where
		T: Display,
	{
		if !self.debug {
			return;
		}

		self.log_impl(
			"DEBUG",
			self.colors.debug,
			caller,
			&message.to_string(),
		);
	}
}

#[macro_export]
macro_rules! error {
	($logger:expr, $caller:expr, $($arg:tt)*) => ({
		$logger.e($caller, format!($($arg)*))
	});
}
#[macro_export]
macro_rules! warn {
	($logger:expr, $caller:expr, $($arg:tt)*) => ({
		$logger.w($caller, format!($($arg)*))
	});
}
#[macro_export]
macro_rules! info {
	($logger:expr, $caller:expr, $($arg:tt)*) => ({
		$logger.i($caller, format!($($arg)*))
	});
}
#[macro_export]
macro_rules! debug {
	($logger:expr, $caller:expr, $($arg:tt)*) => ({
		$logger.d($caller, format!($($arg)*))
	});
}

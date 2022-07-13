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

	pub(self) const fn _default() -> Self {
		Self {
			error: Color::Red,
			warn: Color::Yellow,
			info: Color::Green,
			debug: Color::Blue,
		}
	}
}

impl Default for Colors {
	fn default() -> Self {
		Self::_default()
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

pub static mut LOGGER: Logger =
	Logger { debug: false, colors: Colors::_default() };

pub mod macros {
	#[macro_export]
	macro_rules! error {
		($caller:expr, $($arg:tt)*) => ({
			#[allow(unused_unsafe)]
			unsafe {
				$crate::log::LOGGER.e($caller, format!($($arg)*))
			}
		});
	}
	#[macro_export]
	macro_rules! warning {
		($caller:expr, $($arg:tt)*) => ({
			#[allow(unused_unsafe)]
			unsafe {
				$crate::log::LOGGER.w($caller, format!($($arg)*))
			}
		});
	}
	#[macro_export]
	macro_rules! info {
		($caller:expr, $($arg:tt)*) => ({
			#[allow(unused_unsafe)]
			unsafe {
				$crate::log::LOGGER.i($caller, format!($($arg)*))
			}
		});
	}
	#[macro_export]
	macro_rules! debug {
		($caller:expr, $($arg:tt)*) => ({
			#[allow(unused_unsafe)]
			unsafe {
				$crate::log::LOGGER.d($caller, format!($($arg)*))
			}
		});
	}
}

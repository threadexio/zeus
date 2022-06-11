pub use colored::{control, Color, Colorize};

use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Stream {
	Stdout,
	Stderr,
}

impl Default for Stream {
	fn default() -> Self {
		Self::Stderr
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Colors {
	pub error: Color,
	pub warn: Color,
	pub info: Color,
	pub debug: Color,
}

impl Default for Colors {
	fn default() -> Self {
		ColorsBuilder::new().build()
	}
}

#[derive(Debug, Clone, Copy)]
pub struct ColorsBuilder {
	pub error: Color,
	pub warn: Color,
	pub info: Color,
	pub debug: Color,
}

impl Default for ColorsBuilder {
	fn default() -> Self {
		Self {
			error: Color::Red,
			warn: Color::Yellow,
			info: Color::Green,
			debug: Color::Blue,
		}
	}
}

impl ColorsBuilder {
	pub fn new() -> Self {
		Self { ..Default::default() }
	}

	pub fn build(self) -> Colors {
		Colors {
			error: self.error,
			warn: self.warn,
			info: self.info,
			debug: self.debug,
		}
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

#[derive(Debug, Default)]
pub struct Logger {
	pub colors: Colors,

	pub debug: bool,

	pub out: Stream,
}

impl Logger {
	fn log_impl(
		&self,
		level: &str,
		c: Color,
		caller: &str,
		data: &str,
	) {
		let log_format = format!(
			" {: <5} {} {} {}",
			level.to_string().color(c).bold(),
			caller.to_string().bright_white().bold(),
			"│".bright_black(),
			data.to_string().bright_white()
		);

		use Stream::*;
		match self.out {
			Stdout => println!("{}", log_format),
			Stderr => eprintln!("{}", log_format),
		}
	}

	pub fn e<C, D>(&self, caller: C, message: D)
	where
		C: Display,
		D: Display,
	{
		self.log_impl(
			"ERROR",
			self.colors.error,
			&caller.to_string(),
			&message.to_string(),
		);
	}
	pub fn w<C, D>(&self, caller: C, message: D)
	where
		C: Display,
		D: Display,
	{
		self.log_impl(
			"WARN",
			self.colors.warn,
			&caller.to_string(),
			&message.to_string(),
		);
	}
	pub fn i<C, D>(&self, caller: C, message: D)
	where
		C: Display,
		D: Display,
	{
		self.log_impl(
			"INFO",
			self.colors.info,
			&caller.to_string(),
			&message.to_string(),
		);
	}
	pub fn d<C, D>(&self, caller: C, message: D)
	where
		C: Display,
		D: Display,
	{
		if !self.debug {
			return;
		}

		self.log_impl(
			"DEBUG",
			self.colors.debug,
			&caller.to_string(),
			&message.to_string(),
		);
	}
}

#[macro_export]
macro_rules! log_error {
	($logger:tt, $caller:expr, $($arg:tt)*) => ({
		let mut m = format!($($arg)*);
		m.push('\n');
		$logger.e($caller, m)
	});
}

#[macro_export]
macro_rules! log_warn {
	($logger:tt, $caller:expr, $($arg:tt)*) => ({
		let mut m = format!($($arg)*);
		m.push('\n');
		$logger.w($caller, m)
	});
}

#[macro_export]
macro_rules! log_info {
	($logger:tt, $caller:expr, $($arg:tt)*) => ({
		let mut m = format!($($arg)*);
		m.push('\n');
		$logger.i($caller, m)
	});
}

#[macro_export]
macro_rules! log_debug {
	($logger:tt, $caller:expr, $($arg:tt)*) => ({
		let mut m = format!($($arg)*);
		m.push('\n');
		$logger.d($caller, m)
	});
}

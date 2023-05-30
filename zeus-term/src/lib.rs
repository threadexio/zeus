use std::fmt::Display;
use std::io::{self, prelude::*, Stderr, Stdin, Stdout};
use std::str;

use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(
	Debug,
	Clone,
	Copy,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Serialize,
	Deserialize,
)]
pub enum Level {
	Fatal,
	Error,
	Warn,
	Info,
	Debug,
	Trace,
}

#[derive(Debug)]
pub struct Terminal {
	t_in: Stdin,
	t_out: Stdout,
	t_err: Stderr,

	max_level: Level,

	terminal: bool,
	interactive: bool,
}

impl Terminal {
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		let interactive = atty::is(atty::Stream::Stdin);
		let terminal = atty::is(atty::Stream::Stderr);

		let mut term = Self {
			t_in: io::stdin(),
			t_out: io::stdout(),
			t_err: io::stderr(),
			max_level: Level::Info,
			terminal,
			interactive,
		};

		if !term.is_terminal() {
			term.set_color_enabled(false);
		}

		term
	}

	pub fn is_terminal(&self) -> bool {
		self.terminal
	}

	pub fn is_interactive(&self) -> bool {
		self.interactive
	}

	pub fn set_interactive(&mut self, yes: bool) {
		self.interactive = yes;
	}

	pub fn set_color_enabled(&mut self, enabled: bool) {
		colored::control::set_override(enabled)
	}

	pub fn raw_in(&mut self) -> &mut Stdin {
		&mut self.t_in
	}

	pub fn raw_out(&mut self) -> &mut Stdout {
		&mut self.t_out
	}

	pub fn raw_err(&mut self) -> &mut Stderr {
		&mut self.t_err
	}
}

impl Terminal {
	pub fn read_line(&mut self, hint: Option<usize>) -> String {
		let mut s = String::with_capacity(hint.unwrap_or(16));

		if !self.is_interactive() {
			return s;
		}

		let _ = self.t_in.read_line(&mut s);

		s.trim().to_string()
	}

	pub fn write<M>(&mut self, m: M)
	where
		M: Display,
	{
		let _ = self.t_err.write_all(format!("{m}").as_bytes());
		let _ = self.t_err.flush();
	}

	pub fn writeln<M>(&mut self, m: M)
	where
		M: Display,
	{
		let _ = self.t_err.write_all(format!("{m}\n").as_bytes());
	}
}

impl Terminal {
	pub fn confirm<M>(&mut self, message: M, default: bool) -> bool
	where
		M: Display,
	{
		self.write(format!(
			"{} [{}] ",
			Self::log_fmt(Level::Info, message.to_string().bold()),
			if default {
				"Y/n"
			} else {
				"y/N"
			}
			.dimmed()
		));

		if !self.is_interactive() || !self.is_terminal() {
			self.writeln("");
			return default;
		}

		let answer = self.read_line(Some(8));

		match answer.as_str() {
			"" => default,
			"y" | "Y" | "yes" | "YES" => true,
			_ => false,
		}
	}
}

impl Terminal {
	fn log_fmt<M>(level: Level, message: M) -> String
	where
		M: Display,
	{
		format!(
			" {} {message}",
			match level {
				Level::Trace => "==".bright_white().bold(),
				Level::Debug => "--".bright_white().dimmed(),
				Level::Info => "::".bright_cyan().bold(),
				Level::Warn => "!!".yellow().bold(),
				Level::Error => "**".red().bold(),
				Level::Fatal => "**".bright_red().bold(),
			}
		)
	}

	fn imp_log<M>(&mut self, level: Level, message: M)
	where
		M: Display,
	{
		if level <= self.max_level {
			self.writeln(Self::log_fmt(level, message));
		}
	}

	pub fn set_log_level(&mut self, level: Level) {
		self.max_level = level;
	}

	#[inline]
	pub fn trace<M>(&mut self, message: M)
	where
		M: Display,
	{
		self.imp_log(Level::Trace, message)
	}

	#[inline]
	pub fn debug<M>(&mut self, message: M)
	where
		M: Display,
	{
		self.imp_log(Level::Debug, message)
	}

	#[inline]
	pub fn info<M>(&mut self, message: M)
	where
		M: Display,
	{
		self.imp_log(Level::Info, message)
	}

	#[inline]
	pub fn warn<M>(&mut self, message: M)
	where
		M: Display,
	{
		self.imp_log(Level::Warn, message)
	}

	#[inline]
	pub fn error<M>(&mut self, message: M)
	where
		M: Display,
	{
		self.imp_log(Level::Error, message)
	}

	#[inline]
	pub fn fatal<M>(&mut self, message: M)
	where
		M: Display,
	{
		self.imp_log(Level::Fatal, message)
	}
}
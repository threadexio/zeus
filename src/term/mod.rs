#![allow(dead_code)]
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::str;

use anyhow::{bail, Result};
use colored::Colorize;

#[derive(
	Debug,
	Clone,
	Copy,
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

#[derive(Debug)]
pub struct Terminal {
	t_in: io::Stdin,
	t_out: io::Stdout,
	t_err: io::Stderr,

	read_buf: Vec<u8>,
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
			read_buf: Vec::with_capacity(2048),
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

	#[allow(clippy::missing_safety_doc)]
	pub unsafe fn raw_in(&mut self) -> &mut io::Stdin {
		&mut self.t_in
	}

	#[allow(clippy::missing_safety_doc)]
	pub unsafe fn raw_out(&mut self) -> &mut io::Stdout {
		&mut self.t_out
	}

	#[allow(clippy::missing_safety_doc)]
	pub unsafe fn raw_err(&mut self) -> &mut io::Stderr {
		&mut self.t_err
	}
}

impl Terminal {
	pub fn read_line(&mut self, max_len: usize) -> Result<String> {
		if !self.is_interactive() {
			bail!("Input is not a terminal")
		}

		if max_len > self.read_buf.len() {
			self.read_buf.resize(max_len, 0);
		}
		let line_bytes = &mut self.read_buf[..max_len];

		let i = self.t_in.read(line_bytes)?;
		if i == 0 {
			bail!(io::Error::from(io::ErrorKind::UnexpectedEof));
		}

		let mut line = str::from_utf8(line_bytes)?;
		if let Some(end) = line.find('\0') {
			line = &line[..end];
		}

		let line = line.trim();

		Ok(line.to_string())
	}

	pub fn write<M>(&mut self, m: M) -> io::Result<()>
	where
		M: fmt::Display,
	{
		self.t_err.write_all(format!("{m}").as_bytes())?;
		self.t_err.flush()?;
		Ok(())
	}

	pub fn writeln<M>(&mut self, m: M) -> io::Result<()>
	where
		M: fmt::Display,
	{
		self.t_err.write_all(format!("{m}\n").as_bytes())?;
		Ok(())
	}
}

impl Terminal {
	pub fn confirm<M>(
		&mut self,
		message: M,
		default: bool,
	) -> Result<bool>
	where
		M: fmt::Display,
	{
		if !self.is_terminal() {
			bail!("Output is not a terminal")
		}

		if !self.is_interactive() {
			return Ok(default);
		}

		self.write(format!(
			"{} [{}] ",
			Self::log_fmt(Level::Info, message.to_string().bold()),
			if default {
				"Y/n".bold()
			} else {
				"y/N".bold()
			}
		))?;

		let answer = self.read_line(8)?;

		match answer.as_str() {
			"" => Ok(default),
			"y" | "Y" | "yes" | "YES" => Ok(true),
			_ => Ok(false),
		}
	}

	pub fn prompt<M, D>(
		&mut self,
		message: M,
		default: D,
	) -> Result<String>
	where
		M: fmt::Display,
		D: fmt::Display,
	{
		if !self.is_terminal() {
			bail!("Output is not a terminal")
		}

		if !self.is_interactive() {
			return Ok(default.to_string());
		}

		self.write(format!(
			"{} ({}) ",
			Self::log_fmt(Level::Info, message.to_string().bold()),
			default.to_string().bold()
		))?;

		let answer = self.read_line(2048)?;
		Ok(answer)
	}
}

impl Terminal {
	fn log_fmt<M>(level: Level, message: M) -> String
	where
		M: fmt::Display,
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

	fn imp_log<M>(&mut self, level: Level, message: M) -> Result<()>
	where
		M: fmt::Display,
	{
		if level <= self.max_level {
			self.writeln(Self::log_fmt(level, message))?;
		}

		Ok(())
	}

	pub fn set_log_level(&mut self, level: Level) {
		self.max_level = level;
	}

	#[inline]
	pub fn trace<M>(&mut self, message: M) -> Result<()>
	where
		M: fmt::Display,
	{
		self.imp_log(Level::Trace, message)
	}

	#[inline]
	pub fn debug<M>(&mut self, message: M) -> Result<()>
	where
		M: fmt::Display,
	{
		self.imp_log(Level::Debug, message)
	}

	#[inline]
	pub fn info<M>(&mut self, message: M) -> Result<()>
	where
		M: fmt::Display,
	{
		self.imp_log(Level::Info, message)
	}

	#[inline]
	pub fn warn<M>(&mut self, message: M) -> Result<()>
	where
		M: fmt::Display,
	{
		self.imp_log(Level::Warn, message)
	}

	#[inline]
	pub fn error<M>(&mut self, message: M) -> Result<()>
	where
		M: fmt::Display,
	{
		self.imp_log(Level::Error, message)
	}

	#[inline]
	pub fn fatal<M>(&mut self, message: M) -> Result<()>
	where
		M: fmt::Display,
	{
		self.imp_log(Level::Fatal, message)
	}
}

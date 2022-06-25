pub use colored::{control, Color, Colorize};

use std::collections::HashMap;
use std::fmt::Display;

use std::io;

use std::io::Read;
use std::io::Write;

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
impl Logger {
	pub fn new(stream: Stream, colors: Colors) -> Self {
		Self { out: stream, colors, debug: false }
	}

	fn get_output(&self) -> Box<dyn Write> {
		use Stream::*;
		match self.out {
			Stdout => Box::new(io::stdout()),
			Stderr => Box::new(io::stderr()),
		}
	}

	fn log_impl(
		&self,
		level: &str,
		c: Color,
		caller: &str,
		data: &str,
	) {
		let log_format = format!(
			"{} {: <5} {} {} {}",
			"[".bright_black(),
			level.to_string().color(c).bold(),
			caller.to_string().bright_white().bold(),
			"]".bright_black(),
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

	pub fn prompt<T>(&self, message: T) -> io::Result<String>
	where
		T: Display,
	{
		let mut stream = self.get_output();

		write!(
			stream,
			"{} {}\n{0} ",
			"=>".color(self.colors.info),
			message
		)?;
		stream.flush()?;

		let mut input = String::with_capacity(16);
		io::stdin().read_line(&mut input)?;

		Ok(input)
	}

	pub fn yes_no_question<T>(
		&self,
		question: T,
		default: bool,
	) -> io::Result<bool>
	where
		T: Display,
	{
		let mut stream = self.get_output();

		write!(
			stream,
			"{} {} [{}] ",
			"=>".color(self.colors.info),
			question,
			match default {
				true => "Y/n",
				false => "y/N",
			}
			.bright_white()
			.bold(),
		)?;
		stream.flush()?;

		let mut answer: [u8; 1] = [0; 1];
		io::stdin().read(&mut answer)?;

		match answer[0] as char {
			'y' | 'Y' => Ok(true),
			'n' | 'N' => Ok(false),
			'\n' => Ok(default),
			_ => Ok(false),
		}
	}

	pub fn question<'a, T, A>(
		&self,
		message: T,
		answers: Vec<&'a A>,
		default: &'a A,
		answers_per_line: usize,
	) -> io::Result<Vec<&'a A>>
	where
		T: Display,
		A: Display + ?Sized,
	{
		let mut stream = self.get_output();

		writeln!(
			stream,
			"{} {} [{}]",
			"=>".color(self.colors.info),
			message,
			default.to_string().bright_white().bold()
		)?;

		let mut numbered_answers: HashMap<usize, &A> = HashMap::new();
		for (index, answer) in answers.iter().enumerate() {
			numbered_answers.insert(index, answer);
			write!(
				stream,
				"   {}) {}{}",
				index.to_string().color(self.colors.warn),
				answer,
				match index % answers_per_line {
					3 => "\n",
					_ => "",
				}
			)?;
		}
		write!(stream, "\n{} ", "=>".color(self.colors.info))?;
		stream.flush()?;

		let mut input = String::with_capacity(16);
		io::stdin().read_line(&mut input)?;

		let mut ret: Vec<&A> = Vec::new();

		if input.trim().is_empty() {
			return Ok(ret);
		}

		for answer_number_str in input.trim().split_ascii_whitespace()
		{
			let answer_number: usize = match answer_number_str.parse()
			{
				Ok(v) => v,
				Err(_) => continue,
			};

			if let Some(answer) = numbered_answers.get(&answer_number)
			{
				ret.push(answer)
			}
		}

		Ok(ret)
	}
}

#[macro_export]
macro_rules! log_error {
	($logger:tt, $caller:expr, $($arg:tt)*) => ({
		$logger.e($caller, format!($($arg)*))
	});
}

#[macro_export]
macro_rules! log_warn {
	($logger:tt, $caller:expr, $($arg:tt)*) => ({
		$logger.w($caller, format!($($arg)*))
	});
}

#[macro_export]
macro_rules! log_info {
	($logger:tt, $caller:expr, $($arg:tt)*) => ({
		$logger.i($caller, format!($($arg)*))
	});
}

#[macro_export]
macro_rules! log_debug {
	($logger:tt, $caller:expr, $($arg:tt)*) => ({
		$logger.d($caller, format!($($arg)*))
	});
}

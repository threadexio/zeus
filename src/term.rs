use std::collections::HashMap;
use std::fmt::Display;
use std::io;
use std::io::prelude::*;

pub use colored::{control, Color, ColoredString, Colorize};

use crate::log::Logger;

#[derive(Debug)]
pub struct Terminal {
	pub input: io::Stdin,
	pub output: io::Stdout,
	pub error: io::Stderr,

	pub log: Logger,
}

#[allow(dead_code)]
impl Terminal {
	pub fn new(log: Logger) -> Self {
		Self {
			input: io::stdin(),
			output: io::stdout(),
			error: io::stderr(),
			log,
		}
	}

	pub fn color(&mut self, enabled: bool) {
		control::set_override(enabled)
	}

	pub fn reset_color(&mut self) {
		control::unset_override()
	}

	pub fn prompt<T>(&mut self, message: T) -> io::Result<String>
	where
		T: Display,
	{
		write!(
			self.output,
			"{} {}\n{0} ",
			"=>".green().bold(),
			message
		)?;
		self.output.flush()?;

		let mut input = String::with_capacity(16);
		self.input.read_line(&mut input)?;

		Ok(input)
	}

	pub fn yes_no_question<T>(
		&mut self,
		question: T,
		default: bool,
	) -> io::Result<bool>
	where
		T: Display,
	{
		write!(
			self.output,
			"{} {} [{}] ",
			"=>".green().bold(),
			question,
			match default {
				true => "Y/n",
				false => "y/N",
			}
			.bright_white()
			.bold(),
		)?;
		self.output.flush()?;

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
		&mut self,
		message: T,
		answers: Vec<&'a A>,
		default: &'a A,
		answers_per_line: usize,
	) -> io::Result<Option<Vec<&'a A>>>
	where
		T: Display,
		A: Display + ?Sized,
	{
		writeln!(
			self.output,
			"{} {} [{}]",
			"=>".green().bold(),
			message,
			default.to_string().bright_white().bold()
		)?;

		let mut numbered_answers: HashMap<usize, &A> = HashMap::new();
		for (index, answer) in answers.iter().enumerate() {
			numbered_answers.insert(index, answer);
			write!(
				self.output,
				"   {}) {}{}",
				index.to_string().yellow(),
				answer,
				if index % answers_per_line == answers_per_line - 1 {
					"\n"
				} else {
					""
				}
			)?;
		}
		write!(self.output, "\n{} ", "=>".green().bold())?;
		self.output.flush()?;

		let mut input = String::with_capacity(16);
		self.input.read_line(&mut input)?;

		let mut ret: Vec<&A> = Vec::new();

		if input.trim().is_empty() {
			return Ok(None);
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

		Ok(Some(ret))
	}

	pub fn list<H, I, T>(
		&mut self,
		header: H,
		items: I,
		items_per_line: usize,
	) -> io::Result<()>
	where
		H: Display,
		T: Display,
		I: Iterator<Item = T>,
	{
		writeln!(self.output, "{} {}", "=>".green().bold(), header)?;

		for (index, item) in items.enumerate() {
			write!(
				self.output,
				"    {}{}",
				item,
				if index % items_per_line == items_per_line - 1 {
					"\n"
				} else {
					""
				}
			)?;
		}
		writeln!(self.output, "")?;

		Ok(())
	}
}

impl Default for Terminal {
	fn default() -> Self {
		Self {
			input: io::stdin(),
			output: io::stdout(),
			error: io::stderr(),
			log: Default::default(),
		}
	}
}

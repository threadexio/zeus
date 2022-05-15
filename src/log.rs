pub use termcolor::{Color, ColorChoice, ColorSpec};
use termcolor::{StandardStream, WriteColor};

use std::default::Default;
use std::io::Write;

#[allow(dead_code)]
pub enum Stream {
	Stdout,
	Stderr,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Level {
	Error,
	Warn,
	Success,
	Info,
	Verbose,
	Debug,
}

pub struct Logger {
	pub error_color: Color,
	pub warn_color: Color,
	pub success_color: Color,
	pub info_color: Color,
	pub verbose_color: Color,

	pub verbose: bool,

	#[cfg(debug_assertions)]
	pub debug_color: Color,

	out: StandardStream,
}

#[allow(dead_code)]
impl Logger {
	pub fn new(output: Stream, choice: ColorChoice) -> Self {
		Self {
			out: match output {
				Stream::Stdout => StandardStream::stdout(choice),
				Stream::Stderr => StandardStream::stderr(choice),
			},
			..Default::default()
		}
	}

	pub fn v<T>(&mut self, level: Level, data: T)
	where
		T: std::fmt::Display,
	{
		let color: Color;

		#[allow(unreachable_code)]
		match level {
			Level::Error => {
				color = self.error_color;
			},
			Level::Warn => {
				color = self.warn_color;
			},
			Level::Success => {
				color = self.success_color;
			},
			Level::Info => {
				color = self.info_color;
			},
			Level::Verbose => {
				// skip all verbose messages if we are not running in verbose mode
				if !self.verbose {
					return;
				}

				color = self.verbose_color;
			},
			Level::Debug => {
				// skip all debug messages if we are not running in a debug build
				#[cfg(not(debug_assertions))]
				return;

				#[cfg(debug_assertions)]
				{
					color = self.debug_color;
				}
			},
		}

		self.out
			.set_color(
				ColorSpec::new().set_bold(true).set_fg(Some(color)),
			)
			.unwrap();

		write!(
			&mut self.out,
			"{: >8} ",
			format!("{:?}", level).to_uppercase()
		)
		.unwrap();

		let mut clear_spec = ColorSpec::new();
		clear_spec.clear();
		self.out.set_color(&mut clear_spec).unwrap();

		let msg = data.to_string();
		let mut lines = msg.split('\n');

		// the first line should not have additional padding
		if let Some(v) = lines.next() {
			writeln!(&mut self.out, "{}", v).unwrap();
		}

		lines.for_each(|x| {
			writeln!(&mut self.out, "{: >8} {}", "", x).unwrap()
		})
	}
}

#[allow(dead_code)]
impl Default for Logger {
	fn default() -> Self {
		Self {
			error_color: Color::Red,
			warn_color: Color::Yellow,
			success_color: Color::Green,
			info_color: Color::Blue,
			verbose_color: Color::Cyan,

			verbose: false,

			#[cfg(debug_assertions)]
			debug_color: Color::White,

			out: StandardStream::stdout(ColorChoice::Auto),
		}
	}
}

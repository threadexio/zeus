use std::error::Error;
use std::fmt::Display;
use std::result;

#[allow(dead_code)]
pub type Result<T> = result::Result<T, ZeusError>;

#[derive(Debug)]
pub struct ZeusError {
	pub caller: String,
	pub message: String,
}

#[allow(dead_code)]
impl ZeusError {
	pub fn new(caller: String, message: String) -> Self {
		Self {
			caller: caller.to_owned(),
			message: message.to_owned(),
		}
	}
}

impl Display for ZeusError {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		f.write_fmt(format_args!("{}", self.message))
	}
}

impl Error for ZeusError {}

impl From<std::io::Error> for ZeusError {
	fn from(e: std::io::Error) -> Self {
		return ZeusError {
			caller: "system".to_string(),
			message: e.to_string(),
		};
	}
}

impl From<crate::machine::Error> for ZeusError {
	fn from(e: crate::machine::Error) -> Self {
		return ZeusError {
			caller: "RuntimeManager".to_string(),
			message: format!("Error: {}", e),
		};
	}
}

#[macro_export]
macro_rules! zerr {
	($x:expr, $caller:expr, $($arg:tt)*) => {
		match $x {
			Ok(v) => v,
			Err(e) => {
				return Err(ZeusError::new(
					$caller.to_string(),
					format!("{}: {}", format!($($arg)*), e),
				));
			},
		}
	};
}

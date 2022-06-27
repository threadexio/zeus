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

#[macro_export]
macro_rules! zerr {
	($x:expr, $caller:expr, $msg:expr) => {
		match $x {
			Ok(v) => v,
			Err(e) => {
				return Err(ZeusError::new(
					$caller.to_string(),
					format!("{}: {}", $msg, e),
				));
			},
		}
	};
}

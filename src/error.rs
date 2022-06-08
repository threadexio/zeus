use std::error;
use std::fmt;

#[allow(unused_macros)]
macro_rules! zerr {
	( $x:expr, $caller:expr, $message:expr ) => {
		$x.map_err(|x| x.as_zerr($caller, $message))?
	};
}

#[allow(unused_imports)]
pub(crate) use zerr;

pub struct ZeusError {
	pub caller: String,
	pub message: String,
}

pub type Result<T> = std::result::Result<T, ZeusError>;

#[allow(dead_code)]
impl ZeusError {
	pub fn new<C, D>(caller: C, message: D) -> Self
	where
		C: fmt::Display,
		D: fmt::Display,
	{
		Self {
			caller: caller.to_string(),
			message: message.to_string(),
		}
	}
}

impl fmt::Display for ZeusError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.message)
	}
}

impl fmt::Debug for ZeusError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.message)
	}
}

impl error::Error for ZeusError {}

pub trait AsZerr: error::Error {
	fn as_zerr(
		&self,
		caller: &str,
		extra_message: &str,
	) -> ZeusError {
		ZeusError {
			caller: caller.to_owned(),
			message: format!("{}: {}", extra_message, &self),
		}
	}
}

impl AsZerr for std::io::Error {}

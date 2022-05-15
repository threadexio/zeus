use std::error;
use std::fmt;
use std::io;

#[allow(unused_macros)]
macro_rules! zerr {
	( $x:expr, $m:expr ) => {
		match $x {
			Ok(v) => v,
			Err(e) => {
				return Err(ZeusError::new(
					$m.to_string() + &e.to_string(),
				));
			},
		}
	};
}

#[allow(unused_imports)]
pub(crate) use zerr;

pub type Result<T> = std::result::Result<T, ZeusError>;

pub struct ZeusError {
	pub data: String,
}

#[allow(dead_code)]
impl ZeusError {
	pub fn new<T>(data: T) -> Self
	where
		T: fmt::Display,
	{
		Self { data: data.to_string() }
	}
}

impl fmt::Display for ZeusError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.data)
	}
}

impl fmt::Debug for ZeusError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.data)
	}
}

impl error::Error for ZeusError {}

impl From<io::Error> for ZeusError {
	fn from(e: io::Error) -> Self {
		Self::new(e.to_string())
	}
}

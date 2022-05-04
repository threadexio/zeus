use std::fmt;

#[allow(unused_macros)]
macro_rules! zerr {
	( $x:expr, $m:expr ) => {
		match $x {
			Ok(v) => v,
			Err(e) => {
				return Err(ZeusError::new($m.to_string() + &e.to_string()));
			}
		}
	};
}

#[allow(unused_imports)]
pub(crate) use zerr;

pub struct ZeusError {
	pub data: String,
}

#[allow(dead_code)]
impl ZeusError {
	pub fn new(data: String) -> Self {
		Self { data: data }
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

impl std::error::Error for ZeusError {}

#![allow(unused_macros)]
#![allow(unused_imports)]

pub use std::io::Error;
pub use std::io::ErrorKind;

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! other {
	($($arg:tt)*) => {
		std::io::Error::new(std::io::ErrorKind::Other, format!($($arg)*))
	};
}
pub(crate) use other;

macro_rules! custom {
	($kind:tt, $($arg:tt)*) => {
		std::io::Error::new($kind, format!($($arg)*))
	};
}
pub(crate) use custom;

macro_rules! err {
	($x:expr, $($arg:tt)*) => {
		match $x {
			Ok(v) => v,
			Err(e) =>return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{}: {}", format!($($arg)*), e)))
		}
	};
}
pub(crate) use err;

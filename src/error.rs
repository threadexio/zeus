use std::fmt::{Debug, Display};

use std::error;
use std::result;

pub type Result<T> = result::Result<T, Error>;

pub struct Error {
	inner: Box<dyn Display + Send + Sync + 'static>,
	context: Option<Box<dyn Display + Send + Sync + 'static>>,
}

#[allow(dead_code)]
impl Error {
	pub fn new<E>(e: E) -> Self
	where
		E: Display + Send + Sync + 'static,
	{
		Self { inner: Box::new(e), context: None }
	}

	pub fn new_with_context<E, C>(e: E, c: C) -> Self
	where
		E: Display + Send + Sync + 'static,
		C: Display + Send + Sync + 'static,
	{
		Self { inner: Box::new(e), context: Some(Box::new(c)) }
	}

	pub fn context<C>(mut self, c: C) -> Self
	where
		C: Display + Send + Sync + 'static,
	{
		self.context = Some(Box::new(c));
		self
	}
}

impl<E> From<E> for Error
where
	E: error::Error + Send + Sync + 'static,
{
	fn from(e: E) -> Self {
		Self::new(e)
	}
}

impl Debug for Error {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		write!(f, "{}", self)
	}
}

impl Display for Error {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		match self.context.as_ref() {
			Some(c) => {
				write!(f, "{}: {}", c.as_ref(), self.inner.as_ref())
			},
			None => write!(f, "{}", self.inner.as_ref()),
		}
	}
}

pub trait ErrorExt<T> {
	fn context<C>(self, c: C) -> result::Result<T, Error>
	where
		C: Display + Send + Sync + 'static;
}

impl<T, E> ErrorExt<T> for result::Result<T, E>
where
	E: Display + Send + Sync + 'static,
{
	fn context<C>(self, c: C) -> result::Result<T, Error>
	where
		C: Display + Send + Sync + 'static,
	{
		self.map_err(|e| Error::new(e).context(c))
	}
}

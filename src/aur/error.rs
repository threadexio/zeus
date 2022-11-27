/// An enum representing all errors that can occur
/// when requesting data from an AUR instance.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(transparent)]
	Request(#[from] reqwest::Error),
	#[error("cannot parse response")]
	Parse(#[from] serde_json::Error),
	#[error("query error: {0}")]
	Query(String),
	#[error("invalid response ({0})")]
	InvalidResponse(String),
}

pub type Result<T> = std::result::Result<T, Error>;

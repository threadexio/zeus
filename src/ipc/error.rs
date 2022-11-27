#[derive(Debug, thiserror::Error)]
pub enum Error {
	// TODO: Make these better
	#[error(transparent)]
	Io(#[from] std::io::Error),
	#[error(transparent)]
	Channel(#[from] channels::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

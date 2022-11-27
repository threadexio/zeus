use libloading::Error as LibError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(transparent)]
	RuntimeLoad(LibError),
	#[error(transparent)]
	SymbolNotFound(LibError),
	#[error(transparent)]
	RuntimeUnload(LibError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<LibError> for Error {
	fn from(e: LibError) -> Self {
		match e {
			LibError::DlOpen { .. } | LibError::DlOpenUnknown => {
				Self::RuntimeLoad(e)
			},
			LibError::DlSym { .. } | LibError::DlSymUnknown => {
				Self::SymbolNotFound(e)
			},
			LibError::DlClose { .. } | LibError::DlCloseUnknown => {
				Self::RuntimeUnload(e)
			},
			_ => Self::RuntimeLoad(e),
		}
	}
}

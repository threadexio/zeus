use std::fs::{self, File};
use std::io::{self, Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Lock {
	owner: bool,
	path: PathBuf,
	handle: Option<File>,
}

impl Lock {
	pub fn new<P>(path: P) -> Self
	where
		P: Into<PathBuf>,
	{
		Self { handle: None, owner: false, path: path.into() }
	}

	pub fn path(&self) -> &Path {
		&self.path
	}

	pub fn locked(&self) -> bool {
		self.handle.is_some()
	}

	pub fn lock(&mut self, key: u32) -> Result<()> {
		if self.locked() {
			return Ok(());
		}

		use io::{Read, Write};

		let mut handle = File::options()
			.read(true)
			.write(true)
			.create(true)
			.open(&self.path)?;

		let lock_result = {
			let mut lkey: [u8; 4] = [0; 4];
			match handle.read_exact(&mut lkey) {
				Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
					self.owner = true;
					handle.write_all(&key.to_ne_bytes())?;
				},
				Err(e) => return Err(e),
				Ok(_) => {},
			};

			if !self.owner {
				let lkey = u32::from_ne_bytes(lkey);
				if key != lkey {
					return Err(Error::new(
						ErrorKind::Other,
						"wrong lock key",
					));
				}
			}

			Ok(())
		};

		match lock_result {
			Ok(_) => {
				self.handle = Some(handle);

				Ok(())
			},
			Err(e) => {
				drop(handle);
				let _ = fs::remove_file(&self.path);

				Err(e)
			},
		}
	}

	pub fn unlock(&mut self) -> Result<()> {
		if !self.locked() {
			return Ok(());
		}

		if self.owner {
			fs::remove_file(&self.path)?;
		}

		Ok(())
	}
}

impl Drop for Lock {
	fn drop(&mut self) {
		let _ = self.unlock();
	}
}

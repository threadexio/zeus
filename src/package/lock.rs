use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use fs4::FileExt;

pub struct Lock {
	handle: Option<File>,
	path: PathBuf,
}

// TODO: Custom errors

impl Lock {
	/// Create a new unlocked lock object
	pub fn new(path: &Path) -> Self {
		Self { handle: None, path: path.to_path_buf() }
	}

	/// Engage the lock
	pub fn lock(&mut self) -> io::Result<()> {
		if self.locked() {
			return Ok(());
		}

		let handle = File::options()
			.write(true)
			.create(true)
			.open(&self.path)?;

		handle.try_lock_exclusive()?;

		self.handle = Some(handle);

		Ok(())
	}

	/// Disengage the lock
	pub fn unlock(&mut self) -> io::Result<()> {
		if !self.locked() {
			return Ok(());
		}

		// unwrap is safe here because self.locked() guarantees that
		let handle = self.handle.take().unwrap();
		handle.unlock()?;
		drop(handle);

		Ok(())
	}

	/// Check whether the lock is engaged
	pub fn locked(&self) -> bool {
		self.handle.is_some()
	}

	/// Get the path of the lock on disk
	pub fn path(&self) -> &Path {
		&self.path
	}
}

impl Drop for Lock {
	fn drop(&mut self) {
		let _ = self.unlock();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_lock_functionality() {
		let mut lock1 = Lock::new(Path::new("/tmp/a.lock"));
		let mut lock2 = Lock::new(Path::new("/tmp/a.lock"));

		assert!(!lock1.locked());
		assert!(!lock2.locked());

		lock1.lock().unwrap();

		assert!(lock1.locked());
		assert!(!lock2.locked());

		assert!(lock2.lock().is_err());

		assert!(lock1.locked());
		assert!(!lock2.locked());

		lock1.unlock().unwrap();
		assert!(!lock1.locked());
	}
}

use ::std::{
	fs::File,
	io,
	path::{Path, PathBuf},
};

use ::fs4::FileExt;

pub struct Lock {
	handle: Option<File>,
	path: PathBuf,
}

impl Lock {
	/// Create a new unlocked lock
	pub fn new<P: AsRef<Path>>(path: P) -> Self {
		Self { handle: None, path: path.as_ref().to_path_buf() }
	}

	/// Check whether the lock is engaged
	pub fn locked(&self) -> bool {
		self.handle.is_some()
	}

	/// Get the path of the lock on disk
	pub fn path(&self) -> &Path {
		&self.path
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
		if let Some(handle) = self.handle.take() {
			handle.unlock()?;
		}

		Ok(())
	}
}

impl Drop for Lock {
	fn drop(&mut self) {
		let _ = self.unlock();
	}
}

impl std::fmt::Debug for Lock {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		f.debug_struct("Lock")
			.field("path", &self.path)
			.field("locked", &self.locked())
			.finish()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_lock() {
		let mut lock1 = Lock::new("/tmp/a.lock");
		let mut lock2 = Lock::new("/tmp/a.lock");

		assert!(!lock1.locked());
		assert!(!lock2.locked());

		lock1.lock().expect("failed to lock lock1");

		assert!(lock1.locked());
		assert!(!lock2.locked());

		assert!(lock2.lock().is_err());

		assert!(lock1.locked());
		assert!(!lock2.locked());

		lock1.unlock().expect("failed to unlock lock1");
		assert!(!lock1.locked());
	}
}

use fs4::FileExt;

use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct Lockfile {
	file: fs::File,
}

#[allow(dead_code)]
impl Lockfile {
	pub fn new(path: &Path) -> io::Result<Self> {
		Ok(Self { file: fs::File::create(path)? })
	}

	pub fn lock(&self) -> io::Result<()> {
		self.file.lock_exclusive()
	}

	pub fn try_lock(&self) -> io::Result<()> {
		self.file.try_lock_exclusive()
	}

	pub fn unlock(&self) -> io::Result<()> {
		self.file.unlock()
	}
}

impl Drop for Lockfile {
	fn drop(&mut self) {
		let _ = self.unlock();
	}
}

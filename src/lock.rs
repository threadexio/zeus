use fs4::FileExt;

use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Lockfile {
	file: File,
	path: PathBuf,
}

#[allow(dead_code)]
impl Lockfile {
	pub fn new(path: &Path) -> io::Result<Self> {
		Ok(Self {
			file: if path.exists() {
				File::options().read(true).open(path)?
			} else {
				File::options().create(true).write(true).open(path)?
			},
			path: path.to_path_buf(),
		})
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
		let _ = std::fs::remove_file(&self.path);
	}
}

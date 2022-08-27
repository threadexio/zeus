use std::fs;
use std::path::{Path, PathBuf};

use fs4::FileExt;

use crate::error;

#[derive(Debug)]
pub struct BuildCache {
	dir: PathBuf,
	lockfile: (fs::File, PathBuf, bool),
}

impl BuildCache {
	pub fn new(dir: &str) -> error::Result<Self> {
		let path = Path::new(&dir).canonicalize()?;

		if !path.exists() || !path.is_dir() {
			return Err(error::other!("path is not a directory"));
		}

		let lockfile_path = path.join(".zeus.lock");
		let lockfile_handle;
		if !lockfile_path.exists() {
			lockfile_handle = fs::File::options()
				.create(true)
				.read(true)
				.write(true)
				.open(&lockfile_path)?;
		} else {
			lockfile_handle = fs::File::options()
				.read(true)
				.open(&lockfile_path)?;
		}

		Ok(Self {
			dir: path,
			lockfile: (lockfile_handle, lockfile_path, false),
		})
	}

	pub fn path(&self) -> &Path {
		self.dir.as_path()
	}

	pub fn lock(&mut self) -> error::Result<()> {
		if !self.lockfile.2 {
			self.lockfile.0.lock_exclusive()?;
			self.lockfile.2 = true;
		}

		Ok(())
	}

	pub fn try_lock(&mut self) -> error::Result<()> {
		if !self.lockfile.2 {
			self.lockfile.0.try_lock_exclusive()?;
			self.lockfile.2 = true;
		}

		Ok(())
	}

	pub fn unlock(&mut self) -> error::Result<()> {
		if self.lockfile.2 {
			self.lockfile.0.unlock()?;
			self.lockfile.2 = false;
		}

		Ok(())
	}

	pub fn list_packages(&self) -> error::Result<Vec<String>> {
		let mut ret: Vec<String> = vec![];

		let dir = fs::read_dir(&self.dir)?;
		for entry in dir.filter_map(|x| x.ok()) {
			if !entry.path().is_dir() {
				continue;
			}

			let entry_name =
				entry.file_name().to_string_lossy().to_string();

			match entry_name.as_str() {
				"." | ".." => continue,
				_ => ret.push(entry_name),
			}
		}

		Ok(ret)
	}
}

impl Drop for BuildCache {
	fn drop(&mut self) {
		let _ = self.unlock();
	}
}

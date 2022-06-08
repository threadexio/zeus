use fs4::FileExt;

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};

use crate::error::{zerr, AsZerr, Result};

pub struct Lockfile {
	path: PathBuf,
	file: fs::File,
	pub blocking: bool,
}

impl Lockfile {
	pub fn new(path: &Path) -> Result<Self> {
		Ok(Self {
			path: path.to_path_buf(),
			file: zerr!(
				fs::File::create(path),
				"fs",
				&format!("Cannot create {}", path.display())
			),
			blocking: true,
		})
	}

	pub fn lock(&self) -> Result<()> {
		if self.blocking {
			Ok(zerr!(
				self.file.lock_exclusive(),
				"fs",
				&format!("Cannot lock {}", self.path.display())
			))
		} else {
			Ok(zerr!(
				self.file.try_lock_exclusive(),
				"fs",
				&format!("Cannot lock {}", self.path.display())
			))
		}
	}

	pub fn unlock(&self) -> Result<()> {
		Ok(zerr!(
			self.file.unlock(),
			"fs",
			&format!("Cannot unlock {}", self.path.display())
		))
	}
}

impl Drop for Lockfile {
	fn drop(&mut self) {
		let _ = self.unlock();
	}
}

pub struct LocalListener {
	pub listener: UnixListener,
	path: PathBuf,
}

impl LocalListener {
	pub fn new(path: &Path, mode: Option<u32>) -> Result<Self> {
		let _ = fs::remove_file(path);

		let listener = zerr!(
			UnixListener::bind(path),
			"unix",
			&format!("Cannot bind to {}", path.display())
		);

		if let Some(m) = mode {
			zerr!(
				fs::set_permissions(
					path,
					fs::Permissions::from_mode(m)
				),
				"fs",
				&format!(
					"Cannot change permissions of {}",
					path.display()
				)
			);
		}

		Ok(Self { path: path.to_owned(), listener })
	}
}

impl Drop for LocalListener {
	fn drop(&mut self) {
		let _ = fs::remove_file(self.path.as_path());
	}
}

use fs4::FileExt;

use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};

use crate::error::ZeusError;

impl From<io::Error> for ZeusError {
	fn from(e: io::Error) -> Self {
		ZeusError {
			caller: "system".to_owned(),
			message: e.to_string(),
		}
	}
}

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

pub struct LocalListener {
	pub listener: UnixListener,
	path: PathBuf,
}

#[allow(dead_code)]
impl LocalListener {
	pub fn new(path: &Path, mode: Option<u32>) -> io::Result<Self> {
		let _ = fs::remove_file(path);

		let listener = UnixListener::bind(path)?;

		if let Some(m) = mode {
			fs::set_permissions(path, fs::Permissions::from_mode(m))?;
		}

		Ok(Self { path: path.to_owned(), listener })
	}
}

impl Drop for LocalListener {
	fn drop(&mut self) {
		let _ = fs::remove_file(self.path.as_path());
	}
}

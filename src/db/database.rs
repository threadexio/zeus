use std::fs;
use std::io;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use super::lock::Lock;
use super::pkg::Package;
use super::tools;

pub struct Db<'p> {
	lock: Lock,
	path: PathBuf,
	_p: &'p PhantomData<()>,
}

impl Db<'_> {
	/// Create a new package DB at `path`
	pub fn new<P: AsRef<Path>>(path: P) -> Self {
		Self {
			lock: Lock::new(&path.as_ref().join(".lck")),
			path: path.as_ref().to_path_buf(),
			_p: &PhantomData,
		}
	}

	/// Get the DB root path
	pub fn root(&self) -> &Path {
		&self.path
	}

	/// Get the lockfile path
	pub fn lockfile(&self) -> &Path {
		self.lock.path()
	}

	/// Obtain a lock on the DB
	pub fn lock(&mut self) -> io::Result<()> {
		self.lock.lock()
	}

	/// Release the lock on the DB
	pub fn release(&mut self) -> io::Result<()> {
		self.lock.unlock()
	}
}

impl<'p> Db<'p> {
	fn pkg_name_valid(name: &str) -> bool {
		!name.contains('/') && !name.contains("..")
	}

	/// Get a package
	pub fn get_pkg(&self, name: &str) -> Option<Package<'p>> {
		if !Self::pkg_name_valid(name) {
			return None;
		}

		let pkg_dir = self.path.join(name).canonicalize().ok()?;

		let pkgbuild_path = pkg_dir.join("PKGBUILD");

		if !pkg_dir.exists()
			|| !pkg_dir.is_absolute()
			|| !pkg_dir.is_dir()
			|| pkg_dir.parent() != Some(self.root())
			|| !pkgbuild_path.exists()
		{
			None
		} else {
			Some(Package {
				name: name.to_string(),
				path: pkg_dir,
				_p: &PhantomData,
			})
		}
	}

	/// Clones the package
	pub fn add_pkg(
		&mut self,
		name: &str,
		url: &str,
	) -> io::Result<Package<'p>> {
		if !Self::pkg_name_valid(name) {
			return Err(io::ErrorKind::InvalidInput.into());
		}

		let output = tools::Git::default()
			.at(self.root())
			.attach(true)
			.clone()
			.repository(url)
			.directory(&self.root().join(name))
			.wait()?;

		if !output.status.success() {
			return Err(io::ErrorKind::Other.into());
		}

		match self.get_pkg(name) {
			Some(v) => Ok(v),
			None => Err(io::ErrorKind::NotFound.into()),
		}
	}

	/// Remove a package completely
	pub fn remove_pkg(&mut self, pkg: Package<'_>) -> io::Result<()> {
		fs::remove_dir_all(pkg.path())?;

		Ok(())
	}

	/// List all packages
	pub fn list_pkgs(&mut self) -> io::Result<Vec<Package<'p>>> {
		let mut pkgs = vec![];

		let dir = self.root().read_dir()?;

		for entry in dir.filter_map(|x| x.ok()) {
			match self.get_pkg(&entry.file_name().to_string_lossy()) {
				Some(v) => pkgs.push(v),
				None => continue,
			};
		}

		Ok(pkgs)
	}
}

impl Drop for Db<'_> {
	fn drop(&mut self) {
		let _ = self.release();
	}
}

impl std::fmt::Debug for Db<'_> {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		f.debug_struct("Db")
			.field("path", &self.path)
			.field("lock", &self.lock)
			.finish()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_package_path_validation() {
		let pstore = Db::new("/tmp");

		assert!(matches!(pstore.get_pkg("/tmp/"), None));
		assert!(matches!(pstore.get_pkg("/tmp/../"), None));
		assert!(matches!(pstore.get_pkg("/tmp/../../bin"), None));
	}
}

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use fs4::FileExt;

pub use crate::aur::Package;

pub struct PackageStore {
	root: PathBuf,
	lock_handle: fs::File,
}

impl PackageStore {
	pub fn new(root: &Path) -> io::Result<Self> {
		let path = root.canonicalize()?;

		if !path.exists() || !path.is_dir() {
			return Err(io::ErrorKind::NotFound.into());
		}

		let lock_path = path.join(".zeus.lock");

		// BUG: TOC TOU bug here
		let lock_handle;
		if !lock_path.exists() {
			lock_handle = fs::File::options()
				.create(true)
				.read(true)
				.write(true)
				.open(&lock_path)?;
		} else {
			lock_handle =
				fs::File::options().read(true).open(&lock_path)?;
		}

		Ok(Self { root: path, lock_handle })
	}

	pub fn root(&self) -> &Path {
		self.root.as_path()
	}

	pub fn lock(&mut self) -> io::Result<()> {
		self.lock_handle.try_lock_exclusive()?;

		Ok(())
	}

	pub fn unlock(&mut self) -> io::Result<()> {
		self.lock_handle.unlock()?;

		Ok(())
	}

	fn package_name_path(
		&self,
		package_name: &str,
	) -> io::Result<PathBuf> {
		self.root().join(package_name).canonicalize()
	}

	fn check_package_dir_ok(&self, p: &Path) -> bool {
		p.starts_with(self.root())
			&& p.is_absolute()
			&& p.exists()
			&& p.is_dir()
	}

	pub fn list(&self) -> io::Result<Vec<Package>> {
		let dir = fs::read_dir(&self.root)?;

		let mut pkgs = vec![];

		for entry in dir.filter_map(|x| x.ok()) {
			match entry.file_type() {
				Ok(v) => {
					if !v.is_dir() {
						continue;
					}
				},
				Err(_) => continue,
			};

			pkgs.push(Package {
				name: entry.file_name().to_string_lossy().to_string(),
				..Default::default()
			});
		}

		Ok(pkgs)
	}

	pub fn exists(&self, package_name: &str) -> io::Result<bool> {
		let pkg_dir = self.package_name_path(package_name)?;

		Ok(self.check_package_dir_ok(&pkg_dir))
	}
}

impl Drop for PackageStore {
	fn drop(&mut self) {
		let _ = self.unlock();
	}
}

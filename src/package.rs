use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use fs4::FileExt;
use nix::sys::stat::{umask, Mode};

use crate::aur::Aur;
pub use crate::aur::Package;

use crate::error::*;

// TODO: Make wrappers around pacman, makepkg and git with the builder pattern

pub struct PackageStore {
	root: PathBuf,
	lock_handle: fs::File,
}

#[allow(dead_code)]
impl PackageStore {
	pub fn new(root: &Path) -> Result<Self> {
		let path = root.canonicalize()?;

		if !path.is_dir() {
			return Err(Error::new(
				"Build directory path must be a directory",
			));
		}

		let lock_path = path.join(".zeus.lock");

		umask(Mode::S_IRWXO);

		let lock_handle = fs::File::options()
			.create(true)
			.write(true)
			.open(lock_path)?;

		Ok(Self { root: path, lock_handle })
	}

	pub fn root(&self) -> &Path {
		self.root.as_path()
	}

	pub fn lock(&mut self) -> Result<()> {
		self.lock_handle.try_lock_exclusive()?;

		Ok(())
	}

	pub fn unlock(&mut self) -> Result<()> {
		self.lock_handle.unlock()?;

		Ok(())
	}

	fn package_path(&self, package_name: &str) -> PathBuf {
		self.root().join(package_name)
	}

	fn check_dir(&self, p: &Path) -> bool {
		p.starts_with(self.root())
			&& p.is_absolute()
			&& p.exists()
			&& p.is_dir()
	}

	pub fn exists(&self, package_name: &str) -> bool {
		self.check_dir(&self.package_path(package_name))
	}

	pub fn list(&self) -> Result<Vec<Package>> {
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

			pkgs.push(Package::new(
				entry.file_name().to_string_lossy().to_string(),
			));
		}

		Ok(pkgs)
	}

	pub fn package(&self, package_name: &str) -> Option<Package> {
		if !self.exists(package_name) {
			return None;
		}

		Some(Package::new(package_name.to_string()))
	}

	/// Clone package from AUR
	pub fn clone_package(
		&mut self,
		aur: &Aur,
		package_name: &str,
	) -> Result<Package> {
		let cmd = Command::new("git")
			.args(&["clone", "--"])
			.arg(format!("{}/{}.git", aur.get_url(), package_name))
			.arg(package_name)
			.current_dir(&self.root)
			.stdin(Stdio::inherit())
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit())
			.status()?;

		if !cmd.success() {
			return Err(Error::new(format!(
				"git failed with: {}",
				cmd.code().unwrap_or_default(),
			)));
		}

		match self.package(package_name) {
			Some(v) => Ok(v),
			None => Err(Error::new("unable to find package")),
		}
	}

	/// Build package with makepkg
	pub fn build_package(
		&mut self,
		package: &Package,
		extra_args: &[&str],
	) -> Result<()> {
		let cmd = Command::new("makepkg")
			.args(&["-s"])
			.args(extra_args)
			.current_dir(self.package_path(&package.name))
			.stdin(Stdio::inherit())
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit())
			.status()?;

		if !cmd.success() {
			return Err(Error::new(format!(
				"makepkg failed with: {}",
				cmd.code().unwrap_or_default(),
			)));
		}

		Ok(())
	}

	/// Get package installation files
	pub fn install_package(
		&mut self,
		package: &Package,
	) -> Result<Vec<PathBuf>> {
		let pkg_dir = self.package_path(&package.name);

		let cmd = Command::new("makepkg")
			.args(&["--packagelist"])
			.current_dir(&pkg_dir)
			.stdin(Stdio::null())
			.stdout(Stdio::piped())
			.stderr(Stdio::null())
			.output()?;

		if !cmd.status.success() {
			return Err(Error::new(format!(
				"makepkg failed with: {}",
				cmd.status.code().unwrap_or_default(),
			)));
		}

		let mut files = vec![];

		for i in String::from_utf8_lossy(&cmd.stdout).lines() {
			if let Some(k) = Path::new(i).strip_prefix(&pkg_dir).ok()
			{
				files.push(k.to_path_buf());
			}
		}

		Ok(files)
	}

	/// Remove a package from the build directory
	pub fn remove_package(&mut self, package: Package) -> Result<()> {
		let pkg_dir = self.package_path(&package.name);

		if !self.check_dir(&pkg_dir) {
			return Err(Error::new(format!(
				"invalid package directory {}",
				pkg_dir.display()
			)));
		}

		std::fs::remove_dir_all(&pkg_dir)?;

		Ok(())
	}
}

impl Drop for PackageStore {
	fn drop(&mut self) {
		let _ = self.unlock();
	}
}

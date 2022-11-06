use std::fs;
use std::io;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

/*
//pub mod tools;
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

	/// Get package by name
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
			.args(&["-s", "--noconfirm"])
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
	pub fn get_package_files(
		&mut self,
		package: &Package,
	) -> Result<Vec<String>> {
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
			files.push(i.to_string());
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
*/

pub mod tools;

pub mod lock;
use lock::Lock;

#[derive(Debug)]
pub struct Package<'s> {
	path: PathBuf,
	name: String,
	_p: &'s PhantomData<()>,
}

#[allow(dead_code)]
impl Package<'_> {
	/// Returns the absolute path of the package's cloned repo
	pub fn path(&self) -> &Path {
		&self.path
	}

	/// Returns the name of the package
	pub fn name(&self) -> &str {
		&self.name
	}

	/// Update the package
	pub fn update(&mut self) -> io::Result<()> {
		let _output = tools::Git::default()
			.at(self.path())
			.attach(true)
			.pull()
			.wait()?;

		// TODO: Errors

		Ok(())
	}

	/// Build the package
	pub fn build(&mut self, extra_args: &[&str]) -> io::Result<()> {
		let _output = tools::Makepkg::default()
			.at(self.path())
			.attach(true)
			.install_dependencies()
			.verify_source()
			.args(extra_args)
			.wait()?;

		// TODO: Custom errors based on return code

		Ok(())
	}

	/// Returns a list of the package archives
	pub fn get_install_files(&self) -> io::Result<Vec<String>> {
		let output = tools::Makepkg::default()
			.at(self.path())
			.capture()
			.package_list()
			.wait()?;

		let output = String::from_utf8_lossy(&output.stdout);

		let mut files = vec![];

		for l in output.lines() {
			let l = Path::new(l);

			match l.file_name() {
				Some(v) => {
					files.push(v.to_string_lossy().to_string())
				},
				None => continue,
			};
		}

		Ok(files)
	}
}

impl std::fmt::Display for Package<'_> {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		write!(f, "{}", &self.name)
	}
}

pub struct PackageStore<'p> {
	lock: Lock,
	path: PathBuf,
	_p: &'p PhantomData<()>,
}

#[allow(dead_code)]
impl PackageStore<'_> {
	pub fn new(path: &Path) -> Self {
		Self {
			lock: Lock::new(&path.join(".lck")),
			path: path.to_path_buf(),
			_p: &PhantomData,
		}
	}

	pub fn root(&self) -> &Path {
		&self.path
	}

	pub fn lockfile(&self) -> &Path {
		self.lock.path()
	}

	pub fn lock(&mut self) -> io::Result<()> {
		self.lock.lock()
	}

	pub fn release(&mut self) -> io::Result<()> {
		self.lock.unlock()
	}
}

#[allow(dead_code)]
impl<'p> PackageStore<'p> {
	/// Get a package
	pub fn get_pkg(&self, name: &str) -> Option<Package<'p>> {
		let pkg_dir = match self.path.join(name).canonicalize() {
			Ok(v) => v,
			Err(_) => return None,
		};

		let pkg_build = pkg_dir.join("PKGBUILD");

		// TODO: Separate errors
		if !pkg_dir.exists()
			|| !pkg_dir.is_absolute()
			|| !pkg_dir.is_dir()
			|| pkg_dir.parent() != Some(self.root())
			|| !pkg_build.exists()
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
		let _output = tools::Git::default()
			.at(self.root())
			.attach(true)
			.clone()
			.repository(url)
			.directory(&self.root().join(name))
			.wait()?;

		// TODO: Errors

		match self.get_pkg(name) {
			Some(v) => Ok(v),
			None => panic!("cloned packages does not exist"),
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

impl Drop for PackageStore<'_> {
	fn drop(&mut self) {
		let _ = self.release();
	}
}

impl std::fmt::Debug for PackageStore<'_> {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		f.debug_struct("PackageStore")
			.field("path", &self.path)
			.field("lock", &self.lock.path().display())
			.finish()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_package_path_validation() {
		let pstore = PackageStore::new(Path::new("/tmp"));

		assert!(matches!(pstore.get_pkg("/tmp/"), None));
		assert!(matches!(pstore.get_pkg("/tmp/../"), None));
		assert!(matches!(pstore.get_pkg("/tmp/../../bin"), None));
	}
}

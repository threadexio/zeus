mod lock;
pub mod tools;

use ::std::{
	fs, io,
	marker::PhantomData,
	path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Package<'db> {
	db: &'db Db,
	path: PathBuf,
}

impl<'db> Package<'db> {
	/// Return what would be the path
	/// for a package without checking
	/// if it actually exists
	pub(self) unsafe fn pkg_dir_unchecked(
		db: &Db,
		name: &str,
	) -> PathBuf {
		db.root().join(name)
	}

	pub(self) fn new(
		db: &'db Db,
		name: impl AsRef<str>,
	) -> io::Result<Self> {
		let name = name.as_ref();

		let pkg_path = unsafe { Self::pkg_dir_unchecked(db, name) };

		if !pkg_path.exists() {
			return Err(io::ErrorKind::NotFound.into());
		}

		if !pkg_path.is_absolute()
			|| !pkg_path.is_dir()
			|| pkg_path.parent() != Some(db.root())
			|| name.contains(::std::path::MAIN_SEPARATOR)
		{
			return Err(io::ErrorKind::InvalidData.into());
		}

		Ok(Self { db, path: pkg_path })
	}

	/// Get the name of the package.
	pub fn name(&self) -> &str {
		// unwraps are safe here because we guarantee that the
		// package name is valid utf8 in the `new()` constructor
		#[allow(clippy::unwrap_used)]
		self.path.file_name().unwrap().to_str().unwrap()
	}

	/// Get the database the packages lives in.
	pub fn db(&self) -> &Db {
		self.db
	}

	/// Get the package archives if they are built.
	///
	/// This method will return an empty `Vec` if the archives
	/// are not built.
	pub fn files(&self) -> io::Result<Vec<PathBuf>> {
		let output = tools::Makepkg::default()
			.at(&self.path)
			.capture()
			.package_list()
			.wait()?;

		if !output.status.success() {
			return Err(io::Error::new(
				io::ErrorKind::Other,
				format!(
					"makepkg exited with: {}",
					output.status.code().unwrap_or(-1)
				),
			));
		}

		let s = String::from_utf8_lossy(&output.stdout);

		let mut files = vec![];
		for line in s.lines() {
			let file = Path::new(line.trim());
			if file.exists() && file.is_file() {
				files.push(file.to_path_buf());
			}
		}

		Ok(files)
	}
}

#[derive(Debug)]
enum Action {
	Clone { name: String, url: String, upgrade: bool },
	Build { name: String, extra_args: Vec<String> },
	Remove { name: String },
}

/// A transaction containing work to be done
/// on the database.
///
/// # Usage
/// ```rust
///
/// let db = Db::new("/path/to/db").unwrap();
///
/// let t = Transaction::new().build_pkg("zeus-bin");
///
/// db.lock().commit(t).unwrap();
///
/// ```
#[derive(Debug)]
pub struct Transaction<'db> {
	_p: &'db PhantomData<()>,
	actions: Vec<Action>,
}

impl<'db> Transaction<'db> {
	pub fn new() -> Self {
		Self { _p: &PhantomData, actions: vec![] }
	}

	pub fn clone_pkg<S: AsRef<str>, U: AsRef<str>>(
		mut self,
		name: S,
		url: U,
		upgrade: bool,
	) -> Self {
		self.actions.push(Action::Clone {
			name: name.as_ref().to_string(),
			url: url.as_ref().to_string(),
			upgrade,
		});
		self
	}

	pub fn build_pkg<
		S: AsRef<str>,
		I: Iterator<Item = impl AsRef<str>>,
	>(
		mut self,
		name: S,
		extra_args: I,
	) -> Self {
		self.actions.push(Action::Build {
			name: name.as_ref().to_string(),
			extra_args: extra_args
				.map(|x| x.as_ref().to_string())
				.collect(),
		});
		self
	}

	pub fn remove_pkg<S: AsRef<str>>(mut self, name: S) -> Self {
		self.actions
			.push(Action::Remove { name: name.as_ref().to_string() });
		self
	}
}

#[derive(Debug)]
pub struct Db {
	root: PathBuf,
}

impl Db {
	/// Create a new database at `db_root`.
	///
	/// `db_root` must exist and must be a directory.
	pub fn new<P: AsRef<Path>>(db_root: P) -> io::Result<Self> {
		let db_path = Path::new(db_root.as_ref());

		if !db_path.exists() {
			return Err(io::ErrorKind::NotFound.into());
		}

		if !db_path.is_dir() {
			return Err(io::Error::new(
				io::ErrorKind::Other,
				"root path is not a directory",
			));
		}

		{
			use nix::sys::stat::{umask, Mode};

			umask(Mode::S_IRWXO); // umask 007
		}

		Ok(Self { root: db_path.to_path_buf() })
	}

	/// Get the database root path.
	pub fn root(&self) -> &Path {
		&self.root
	}

	/// Lock the database obtaining a [`DbGuard`](DbGuard).
	pub fn lock(&self) -> io::Result<DbGuard> {
		DbGuard::new(self)
	}

	/// This is only supposed to be used from the builder.
	///
	/// Do not ever use it elsewhere.
	pub unsafe fn unlocked_guard(&self) -> DbGuard {
		DbGuard::new_unlocked(self)
	}

	/// Get a package.
	pub fn pkg(&self, name: impl AsRef<str>) -> io::Result<Package> {
		Package::new(self, name)
	}

	/// Get all packages found in the database.
	pub fn list_pkgs(&self) -> io::Result<Vec<Package>> {
		let mut pkgs = vec![];

		for entry in fs::read_dir(self.root())?
			.filter_map(|x| x.ok())
			.filter(|x| x.path().is_dir())
		{
			if let Some(name) = entry.file_name().to_str() {
				if let Ok(pkg) = self.pkg(name) {
					pkgs.push(pkg);
				}
			}
		}

		Ok(pkgs)
	}
}

/// A scope-based lock guard. Whenever `DbGuard` is created a lock is obtained for the database
/// and automatically release when `DbGuard` falls out of scope or its `release()` method is called.
///
/// This ensures all mutating operations will be done while the database is locked.
///
/// # Usage
/// ```rust
///
/// let db = Db::new("/path/to/db").unwrap();
///
/// let mut guard = db.lock().unwrap(); // db gets locked here
/// // ...
/// guard.release(); // db gets unlocked here
///
/// ```
#[derive(Debug)]
pub struct DbGuard<'db> {
	db: &'db Db,
	lock: lock::Lock,
}

impl<'db> DbGuard<'db> {
	pub(self) fn new(db: &'db Db) -> io::Result<Self> {
		let mut x = Self::new_unlocked(db);
		x.lock.lock()?;

		Ok(x)
	}

	pub(self) fn new_unlocked(db: &'db Db) -> Self {
		let lock = lock::Lock::new(db.root().join(".lck"));

		Self { db, lock }
	}

	pub fn release(self) {}

	/// Commit a transaction
	pub fn commit(
		&mut self,
		transaction: Transaction,
	) -> io::Result<()> {
		for action in &transaction.actions {
			match action {
				Action::Build { name, extra_args } => {
					self.imp_build_pkg(name, extra_args)?
				},
				Action::Clone { name, url, upgrade } => {
					self.imp_clone_pkg(name, url, *upgrade)?
				},
				Action::Remove { name } => {
					self.imp_remove_pkg(name)?
				},
			}
		}

		Ok(())
	}

	fn imp_build_pkg(
		&self,
		name: &str,
		extra_args: &[String],
	) -> io::Result<()> {
		let pkg = self.db.pkg(name)?;

		let output = tools::Makepkg::default()
			.at(&pkg.path)
			.attach(true)
			.needed()
			.noconfirm()
			.install_dependencies()
			.force()
			.args(extra_args)
			.wait()?;

		if !output.status.success() {
			return Err(io::Error::new(
				io::ErrorKind::Other,
				format!(
					"makepkg exited with: {}",
					output.status.code().unwrap_or(-1)
				),
			));
		}

		Ok(())
	}

	fn imp_clone_pkg(
		&self,
		name: &str,
		url: &str,
		upgrade: bool,
	) -> io::Result<()> {
		let output;

		if let Ok(pkg) = self.db.pkg(name) {
			if upgrade {
				output = tools::Git::default()
					.at(&pkg.path)
					.attach(true)
					.pull()
					.wait()?;
			} else {
				return Err(io::ErrorKind::AlreadyExists.into());
			}
		} else {
			output = tools::Git::default()
				.attach(true)
				.clone()
				.repository(url)
				.directory(unsafe {
					&Package::pkg_dir_unchecked(self.db, name)
				})
				.wait()?;
		}

		if !output.status.success() {
			return Err(io::Error::new(
				io::ErrorKind::Other,
				format!(
					"git exited with: {}",
					output.status.code().unwrap_or(-1)
				),
			));
		}

		Ok(())
	}

	fn imp_remove_pkg(&self, name: &str) -> io::Result<()> {
		let pkg = self.db.pkg(name)?;
		fs::remove_dir_all(pkg.path)?;
		Ok(())
	}
}

impl std::ops::Deref for DbGuard<'_> {
	type Target = Db;

	fn deref(&self) -> &Self::Target {
		self.db
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const DB_PATH: &str = "/";

	#[test]
	fn pkg_path_traversal() {
		let db = Db::new(DB_PATH).unwrap();

		assert!(matches!(db.pkg("etc"), Ok(_)));
		assert!(matches!(db.pkg("/etc"), Err(_)));
		assert!(matches!(db.pkg("/etc/../etc"), Err(_)));
		assert!(matches!(db.pkg("zeus-bin"), Err(_)));
		assert!(matches!(db.pkg("../../../test"), Err(_)));
		assert!(matches!(db.pkg("/test"), Err(_)));
		assert!(matches!(db.pkg("https://example.com/"), Err(_)));
		assert!(matches!(db.pkg("../././////"), Err(_)));
	}

	#[test]
	fn pkg_functionality() {
		let db = Db::new(DB_PATH).unwrap();

		let pkg = db.pkg("etc").unwrap();

		assert_eq!(pkg.name(), "etc");
	}
}

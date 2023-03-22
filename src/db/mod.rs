#![allow(dead_code)]

use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

mod lock;
use lock::Lock;

mod pkg;
use pkg::Package;

pub mod git;
pub mod makepkg;

#[derive(Debug)]
pub struct Db {
	lock: Lock,
	root: PathBuf,
}

impl Db {
	pub fn new<P>(root: P) -> Result<Self>
	where
		P: AsRef<Path>,
	{
		let root = root.as_ref();
		let root = root.canonicalize()?;

		if !root.is_dir() {
			return Err(Error::new(
				ErrorKind::Other,
				"corrupted database",
			));
		}

		Ok(Self { lock: Lock::new(root.join(".lck")), root })
	}

	pub fn path(&self) -> &Path {
		&self.root
	}

	pub fn lock(&mut self, key: u32) -> Result<()> {
		self.lock.lock(key)?;
		Ok(())
	}

	pub fn unlock(&mut self) -> Result<()> {
		self.lock.unlock()?;
		Ok(())
	}
}

impl Db {
	/// Retrieve a package from the database.
	pub fn package<P>(&self, name: P) -> Result<Package>
	where
		P: AsRef<str>,
	{
		Package::open(&self.root, name.as_ref())
	}

	/// Check whether a package exists in the database.
	pub fn exists<P>(&self, name: P) -> bool
	where
		P: AsRef<str>,
	{
		self.package(name).is_ok()
	}

	/// Iterate through all packages found in the database.
	pub fn list(&mut self) -> Result<impl Iterator<Item = Package>> {
		let i = self
			.root
			.read_dir()?
			.filter_map(|x| x.ok())
			.filter(|x| {
				x.file_type().map(|x| x.is_dir()).unwrap_or(false)
			})
			.filter_map(|x| {
				let name = x.file_name();
				let name = name.to_str()?;
				Package::open(&self.root, name).ok()
			});

		Ok(i)
	}
}

impl Db {
	/// Clone a package into the database
	/// from the remote repo at `source`.
	pub fn clone<P, S>(
		&mut self,
		name: P,
		source: S,
	) -> Result<Package>
	where
		P: Into<String>,
		S: Into<String>,
	{
		let name = name.into();
		let source = source.into();

		if self.exists(&name) {
			return Err(Error::from(ErrorKind::AlreadyExists));
		}

		git::Git::new(
			git::Clone::new().remote(source).directory(&name),
		)
		.attached(true)
		.cwd(&self.root)
		.execute()?;

		self.package(&name)
	}
}

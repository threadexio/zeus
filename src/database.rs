use std::fs;
use std::io::{self, Seek};
use std::path::{Path, PathBuf};

use serde_json;

use crate::error::AsZerr;
use crate::util::Lockfile;

pub use crate::aur::Package;

#[derive(Debug)]
pub enum Error {
	InUse,
	DoesNotExist,
	InvalidData(serde_json::Error),
	Io(io::Error),
}

impl From<std::io::Error> for Error {
	fn from(e: std::io::Error) -> Self {
		Error::Io(e)
	}
}

impl From<serde_json::Error> for Error {
	fn from(e: serde_json::Error) -> Self {
		Error::InvalidData(e)
	}
}

impl std::fmt::Display for Error {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		use Error::*;
		match self {
			InUse => f.write_fmt(format_args!("resource in use")),
			DoesNotExist => f.write_str("does not exist"),
			InvalidData(e) => {
				f.write_fmt(format_args!("invalid data: {}", e))
			},
			Io(e) => f.write_fmt(format_args!("io error: {}", e)),
		}
	}
}

impl std::error::Error for Error {}

impl AsZerr for Error {}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Database {
	path: PathBuf,
	lockfile: Lockfile,
}

impl Database {
	pub fn new(root: &Path) -> Result<Self> {
		let lock = Lockfile::new(&root.join("zeus.lock"))?;

		match lock.try_lock() {
			Ok(v) => v,
			Err(e) => {
				return Err(match e.kind() {
					io::ErrorKind::WouldBlock => Error::InUse,
					_ => Error::Io(e),
				});
			},
		};

		Ok(Self { path: root.to_path_buf(), lockfile: lock })
	}

	pub fn entry(&self, name: &str) -> Result<Entry> {
		let path = self.path.join(name);

		Entry::new(name, path)
	}
}

#[derive(Debug)]
pub struct Entry {
	key: String,
	value: Package,

	path: PathBuf,
	entry_file: Option<fs::File>,
}

impl Entry {
	fn new(key: &str, path: PathBuf) -> Result<Self> {
		let entry_file: Option<fs::File> = fs::File::options()
			.read(true)
			.write(true)
			.create(false)
			.open(&path)
			.ok();

		let mut package = Package::default();
		if entry_file.is_some() {
			let mut file = entry_file.as_ref().unwrap();
			package = serde_json::from_reader(file)?;
			file.seek(io::SeekFrom::Start(0))?;
		}

		Ok(Entry {
			key: String::from(key),
			value: package,
			path,
			entry_file,
		})
	}

	/// Get the entry name
	pub fn name(&self) -> &str {
		&self.key
	}

	/// Check if the entry exists on disk
	pub fn exists(&self) -> bool {
		self.path.exists()
	}

	/// Get the package contained in this entry
	pub fn get(&self) -> &Package {
		&self.value
	}

	/// Get a mutable reference to the package contained in this entry
	pub fn get_mut(&mut self) -> &mut Package {
		&mut self.value
	}

	/// Update the package in this entry
	pub fn update(&mut self, package: Package) -> &mut Self {
		self.value = package;
		self
	}

	/// Save the entry data back to disk, does not need to be called directly
	pub fn save(&mut self) -> Result<&mut Self> {
		if self.entry_file.is_none() {
			self.entry_file = Some(
				fs::File::options()
					.read(true)
					.write(true)
					.create(true)
					.open(&self.path)?,
			);
		}

		let mut file = self.entry_file.as_ref().unwrap();

		file.set_len(0)?;
		serde_json::to_writer_pretty(file, &self.value)?;
		file.seek(io::SeekFrom::Start(0))?;
		file.sync_all()?;

		Ok(self)
	}

	/// Remove the entry from the database
	pub fn remove(mut self) -> Result<()> {
		if !self.exists() {
			return Err(Error::DoesNotExist);
		}

		fs::remove_file(&self.path)?;
		Ok(())
	}
}

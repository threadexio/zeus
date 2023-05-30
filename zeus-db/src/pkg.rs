use std::ffi::OsStr;
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use super::git;
use super::makepkg;

#[derive(Debug)]
pub struct Package<'a> {
	_p: PhantomData<&'a ()>,
	root: PathBuf,
}

impl<'a> Package<'a> {
	pub(super) fn open(db_root: &Path, name: &str) -> Result<Self> {
		let root = db_root.join(name);
		let root = root.canonicalize()?;

		if root.parent() != Some(db_root) || !root.is_dir() {
			return Err(Error::new(
				ErrorKind::Other,
				"corrupted package",
			));
		}

		Ok(Package { _p: PhantomData, root })
	}

	pub fn name(&self) -> &str {
		self.root
			.file_name()
			.expect("a package should always have a directory")
			.to_str()
			.expect("a package should always have a valid utf8 name")
	}

	pub fn path(&self) -> &Path {
		&self.root
	}

	pub fn update(&mut self) -> Result<()> {
		git::Git::new(
			git::Stash::new().action("save").untracked(true),
		)
		.attached(true)
		.cwd(&self.root)
		.execute()?;

		git::Git::new(git::Pull::new())
			.attached(true)
			.cwd(&self.root)
			.execute()?;

		Ok(())
	}

	pub fn build<A, I>(&mut self, extra_args: I) -> Result<()>
	where
		A: AsRef<OsStr>,
		I: Iterator<Item = A>,
	{
		makepkg::Makepkg::new()
			.attached(true)
			.cwd(&self.root)
			.no_confirm(true)
			.needed(true)
			.sync_deps(true)
			.force(true)
			.execute_with(|c| {
				c.args(extra_args);
			})?;

		Ok(())
	}

	pub fn files(&self) -> Result<Vec<PathBuf>> {
		let output = makepkg::Makepkg::new()
			.attached(false)
			.cwd(&self.root)
			.package_list(true)
			.execute()?;

		let output = String::from_utf8_lossy(&output.stdout);

		Ok(output.lines().map(PathBuf::from).collect::<Vec<_>>())
	}

	/// Remove a package from the database.
	pub fn remove(&mut self) -> Result<()> {
		fs::remove_dir_all(&self.root)?;

		Ok(())
	}
}

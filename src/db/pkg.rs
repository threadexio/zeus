use std::ffi::OsStr;
use std::io;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use super::tools;

#[derive(Debug, PartialEq, Eq)]
pub struct Package<'s> {
	pub(super) path: PathBuf,
	pub(super) name: String,
	pub(super) _p: &'s PhantomData<()>,
}

impl Package<'_> {
	/// Returns the absolute path of the package's repo
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
	pub fn build<R, I>(&mut self, extra_args: I) -> io::Result<()>
	where
		R: AsRef<OsStr>,
		I: Iterator<Item = R>,
	{
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

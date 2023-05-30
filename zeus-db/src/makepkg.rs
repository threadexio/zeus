use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;
use std::process::{Command, Output};

#[derive(Debug, Clone, Default)]
pub struct Makepkg {
	cwd: Option<PathBuf>,
	attached: bool,
	clean_build: bool,
	force: bool,
	sync_deps: bool,
	package_list: bool,
	print_src_info: bool,
	needed: bool,
	no_confirm: bool,
}

impl Makepkg {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn cwd(mut self, cwd: impl Into<PathBuf>) -> Self {
		self.cwd = Some(cwd.into());
		self
	}

	pub fn attached(mut self, yes: bool) -> Self {
		self.attached = yes;
		self
	}

	pub fn clean_build(mut self, yes: bool) -> Self {
		self.clean_build = yes;
		self
	}

	pub fn force(mut self, yes: bool) -> Self {
		self.force = yes;
		self
	}

	pub fn sync_deps(mut self, yes: bool) -> Self {
		self.sync_deps = yes;
		self
	}

	pub fn package_list(mut self, yes: bool) -> Self {
		self.package_list = yes;
		self
	}

	pub fn print_src_info(mut self, yes: bool) -> Self {
		self.print_src_info = yes;
		self
	}

	pub fn needed(mut self, yes: bool) -> Self {
		self.needed = yes;
		self
	}

	pub fn no_confirm(mut self, yes: bool) -> Self {
		self.no_confirm = yes;
		self
	}

	pub fn execute(self) -> Result<Output> {
		self.execute_with(|_| {})
	}

	pub fn execute_with<F>(self, f: F) -> Result<Output>
	where
		F: FnOnce(&mut Command),
	{
		let mut c = Command::new("makepkg");

		if self.attached {
			c.arg("-");
		}

		if self.clean_build {
			c.arg("--cleanbuild");
		}

		if self.force {
			c.arg("--force");
		}

		if self.sync_deps {
			c.arg("--syncdeps");
		}

		if self.package_list {
			c.arg("--packagelist");
		}

		if self.print_src_info {
			c.arg("--printsrcinfo");
		}

		if self.needed {
			c.arg("--needed");
		}

		if self.no_confirm {
			c.arg("--noconfirm");
		}

		if let Some(cwd) = self.cwd {
			c.current_dir(cwd);
		}

		use std::process::Stdio;
		if self.attached {
			c.stdin(Stdio::inherit());
			c.stdout(Stdio::inherit());
			c.stderr(Stdio::inherit());
		} else {
			c.stdin(Stdio::null());
			c.stdout(Stdio::piped());
			c.stderr(Stdio::piped());
		}

		f(&mut c);

		let output = c
			.output()
			.map_err(|x| Error::new(ErrorKind::Other, x))?;
		let exit_code = output.status.code().unwrap_or(-1);

		if exit_code == 0 {
			return Ok(output);
		}

		let e = match exit_code {
			2 => "error in configuration file",
			3 => "user specified an invalid option",
			4 => "error in user-supplied function in pkgbuild",
			5 => "failed to create a viable package",
			6 => "a source or auxiliary file specified in the pkgbuild is missing",
			7 => "the pkgdir is missing",
			8 => "failed to install dependencies",
			9 => "failed to remove dependencies",
			10 => "user attempted to run makepkg as root",
			11 => "user lacks permissions to build or install to a given location",
			12 => "error parsing pkgbuild",
			13 => "a package has already been built",
			14 => "the package failed to install",
			15 => "programs necessary to run makepkg are missing",
			16 => "specified gpg key does not exist or failed to sign package",
			_ => "unknown cause of failure",
		};

		Err(Error::new(ErrorKind::Other, e))
	}
}

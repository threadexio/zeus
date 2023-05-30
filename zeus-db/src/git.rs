#![allow(dead_code)]

use std::ffi::OsString;
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;
use std::process::{Command, Output};

pub trait GitCommand {
	fn build_cmd(&self, c: &mut Command) -> Result<()>;
}

pub struct Clone {
	branch: Option<String>,
	directory: Option<OsString>,
	remote: String,
}

impl Clone {
	pub fn new() -> Self {
		Self { branch: None, directory: None, remote: "".into() }
	}

	pub fn branch(mut self, branch: impl Into<String>) -> Self {
		self.branch = Some(branch.into());
		self
	}

	pub fn directory(
		mut self,
		directory: impl Into<OsString>,
	) -> Self {
		self.directory = Some(directory.into());
		self
	}

	pub fn remote(mut self, remote: impl Into<String>) -> Self {
		self.remote = remote.into();
		self
	}
}

impl GitCommand for Clone {
	fn build_cmd(&self, c: &mut Command) -> Result<()> {
		c.arg("clone");

		if let Some(ref branch) = self.branch {
			c.args(["-b", branch]);
		}

		c.arg("--");
		c.arg(&self.remote);

		if let Some(ref directory) = self.directory {
			c.arg(directory);
		}

		Ok(())
	}
}

pub struct Pull {}

impl Pull {
	pub fn new() -> Self {
		Self {}
	}
}

impl GitCommand for Pull {
	fn build_cmd(&self, c: &mut Command) -> Result<()> {
		c.arg("pull");

		Ok(())
	}
}

pub struct Stash {
	action: Option<String>,
	untracked: bool,
}

impl Stash {
	pub fn new() -> Self {
		Self { action: None, untracked: false }
	}

	pub fn action(mut self, action: impl Into<String>) -> Self {
		self.action = Some(action.into());
		self
	}

	pub fn untracked(mut self, yes: bool) -> Self {
		self.untracked = yes;
		self
	}
}

impl GitCommand for Stash {
	fn build_cmd(&self, c: &mut Command) -> Result<()> {
		c.arg("stash");

		if let Some(ref action) = self.action {
			c.arg(action);
		}

		if self.untracked {
			c.arg("-u");
		}

		Ok(())
	}
}

pub struct Git<C> {
	attached: bool,
	cwd: Option<PathBuf>,
	command: C,
}

impl<C> Git<C>
where
	C: GitCommand,
{
	pub fn new(command: C) -> Self {
		Self { attached: true, cwd: None, command }
	}

	pub fn attached(mut self, yes: bool) -> Self {
		self.attached = yes;
		self
	}

	pub fn cwd(mut self, cwd: impl Into<PathBuf>) -> Self {
		self.cwd = Some(cwd.into());
		self
	}

	pub fn execute(self) -> Result<Output> {
		self.execute_with(|_| {})
	}

	pub fn execute_with<F>(self, f: F) -> Result<Output>
	where
		F: FnOnce(&mut Command),
	{
		let mut c = Command::new("git");

		self.command.build_cmd(&mut c)?;

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

		let output = c.output()?;

		if !output.status.success() {
			let err = String::from_utf8_lossy(&output.stderr);
			return Err(Error::new(ErrorKind::Other, err));
		}

		Ok(output)
	}
}

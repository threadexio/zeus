use std::ffi::OsString;
use std::io;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::process::{Command, Output, Stdio};

pub trait CommandExt {
	fn with_stdout(&mut self) -> io::Result<Output>;

	fn embed_command(&mut self, inner: &Self) -> &mut Self;
}

impl CommandExt for Command {
	fn embed_command(&mut self, inner: &Command) -> &mut Self {
		self.arg("env");

		if let Some(wd) = inner.get_current_dir() {
			self.arg("--chdir");
			self.arg(wd);
		}

		self.arg("--");

		for (k, v) in
			inner.get_envs().filter_map(|(k, v)| Some((k, v?)))
		{
			let mut arg = OsString::from(k);
			arg.push("=");
			arg.push(v);

			self.arg(arg);
		}

		self.arg(inner.get_program());
		for arg in inner.get_args() {
			self.arg(arg);
		}

		self
	}

	fn with_stdout(&mut self) -> io::Result<Output> {
		self.stdout(Stdio::piped()).output()
	}
}

fn check_tool_exists(tool: &str) -> io::Result<bool> {
	match Command::new(tool).output() {
		Ok(_) => Ok(true),
		Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
		Err(e) => Err(e),
	}
}

macro_rules! impl_tool {
	($tool_name:ident, $tool_path:expr, fn: [$($fn_name:ident),*]) => {
		pub struct $tool_name(Command);

		#[allow(dead_code)]
		impl $tool_name {
			$(
				impl_tool!(@fn $tool_name, $tool_path, $fn_name);
			)*
		}

		impl Deref for $tool_name {
			type Target = Command;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl DerefMut for $tool_name {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.0
			}
		}
	};
	(@fn $tool_name:ident, $tool_path:expr, _all_) => {
		impl_tool!(@fn $tool_name, $tool_path, new);
		impl_tool!(@fn $tool_name, $tool_path, into_inner);
		impl_tool!(@fn $tool_name, $tool_path, exists);
	};
	(@fn $tool_name:ident, $tool_path:expr, new) => {
		pub fn new() -> Self {
			let mut c = Command::new($tool_path);
			c.stdin(Stdio::inherit())
				.stdout(Stdio::inherit())
				.stderr(Stdio::inherit());
			Self(c)
		}
	};
	(@fn $tool_name:ident, $tool_path:expr, into_inner) => {
		pub fn into_inner(self) -> Command {
			self.0
		}
	};
	(@fn $tool_name:ident, $tool_path:expr, exists) => {
		pub fn exists() -> bool {
			check_tool_exists($tool_path).unwrap_or(false)
		}
	};
	(@fn $tool_name:ident, $tool_path:expr,) => {};
}

impl_tool!(Cargo,        "cargo",        fn: [_all_]);
impl_tool!(Turboinstall, "turboinstall", fn: [_all_]);
impl_tool!(Makepkg,      "makepkg",      fn: [_all_]);
impl_tool!(Tar,          "tar",          fn: [_all_]);
impl_tool!(Fakeroot,     "fakeroot",     fn: [into_inner, exists]);

impl Fakeroot {
	pub fn new(save: Option<&Path>) -> Self {
		let mut c = Command::new("fakeroot");
		c.stdin(Stdio::inherit())
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit());

		if let Some(file) = save {
			c.arg("-s").arg(file).arg("-i").arg(file);
		}

		Self(c)
	}
}

use std::ffi::OsStr;

/// Macro to generate complete wrappers around command line programs
///
/// Syntax:
/// <Tool name>(<default tool path>) {
///     <name of wrapper call>: [
///         (<argument to set>, <value for that argument (optional)>: <type>),
///         <...>
///     ]
/// }
macro_rules! cmd_tool {
	($tool_name:ident($default_path:expr) {
		$(
			$name:ident: [
				$((
					$($arg:expr)?, $($value:ident:$type:ty)?
				)),*
			],
		)*
	}) => {
		pub struct $tool_name(::std::process::Command);


		impl $tool_name {

			$(
				#[allow(unused_mut)]
				pub fn $name(mut self $($(, $value: $type)?),*) -> Self {

					$(
						$(
							self.0.arg($arg);
						)?
						$(
							self.0.arg($value);
						)?
					)*

					self
				}
			)*

			/// Custom argument
			pub fn arg<S: AsRef<OsStr>>(mut self, v: S) -> Self {
				self.0.arg(v);
				self
			}

			/// Custom arguments
			pub fn args<I, S>(mut self, v: I) -> Self
			where
				I: IntoIterator<Item = S>,
				S: AsRef<OsStr>,{
				self.0.args(v);
				self
			}

			/// Custom tool path
			pub fn new(path: &str) -> Self {
				Self(::std::process::Command::new(path))
			}

			/// Attach to the current terminal or to null
			pub fn attach(mut self, terminal: bool) -> Self {
				use ::std::process::Stdio;

				if terminal {
					self.0.stdin(Stdio::inherit());
					self.0.stdout(Stdio::inherit());
					self.0.stderr(Stdio::inherit());
				} else {
					self.0.stdin(Stdio::null());
					self.0.stdout(Stdio::null());
					self.0.stderr(Stdio::null());
				}
				self
			}

			/// Capture stdin,stdout,stderr
			pub fn capture(mut self) -> Self {
				use ::std::process::Stdio;

				self.0.stdin(Stdio::piped());
				self.0.stdout(Stdio::piped());
				self.0.stderr(Stdio::piped());
				self
			}

			/// Set working directory
			pub fn at(mut self, at: &Path) -> Self {
				self.0.current_dir(at);
				self
			}

			/// Into raw std::process::Command
			pub fn into_raw(self) -> ::std::process::Command {
				self.0
			}

			/// From raw std::process::Command
			pub fn from_raw(c: ::std::process::Command) -> Self {
				Self(c)
			}

			/// Run the command and wait for it to complete
			pub fn wait(mut self) -> ::std::io::Result<::std::process::Output> {
				self.0.output()
			}
		}

		impl Default for $tool_name {
			fn default() -> Self {
				Self(::std::process::Command::new($default_path))
			}
		}

		impl std::fmt::Display for $tool_name {
			fn fmt(
				&self,
				f: &mut std::fmt::Formatter<'_>,
			) -> std::fmt::Result {
				write!(f, "{:?}", &self.0)
			}
		}
	};
}

use std::path::Path;

cmd_tool! {
	Pacman("/usr/bin/pacman") {
		database: [("--database",)],
		files: [("--files",)],
		query: [("--query",)],
		remove: [("--remove",)],
		sync: [("--sync",)],
		deptest: [("--deptest",)],
		upgrade: [("--upgrade",)],
		changelog: [("--changelog",)],
		deps: [("--deps",)],
		explicit: [("--explicit",)],
		groups: [("--groups",)],
		info: [("--info",)],
		check: [("--check",)],
		list: [("--list",)],
		foreign: [("--foreign,",)],
		native: [("--native,",)],
		owns: [("--owns", file: &str)],
		file: [("--file", package: &Path)],
		quiet: [("--quiet",)],
		search: [("--search", regex: &str)],
		unrequired: [("--unrequired",)],
		upgrades: [("--upgrades",)],
		cascade: [("--cascade",)],
		nosave: [("--nosave",)],
		recursive: [("--recursive",)],
		unneeded: [("--unneeded",)],
		assume_installed: [("--assume-installed", package: &str)],
		cachedir: [("--cachedir", dir: &Path)],
		color: [("--color", when: &str)],
		config: [("--config", path: &Path)],
		confirm: [("--confirm",)],
		dbonly: [("--dbonly",)],
		debug: [("--debug",)],
		disable_download_timeout: [("--disable-download-timeout",)],
		gpgdir: [("--gpgdir", path: &Path)],
		hookdir: [("--hookdir", dir: &Path)],
		logfile: [("--logfile", path: &Path)],
		noconfirm: [("--noconfirm",)],
		noprogressbar: [("--noprogressbar",)],
		noscriptlet: [("--noscriptlet",)],
		print_format: [("--print-format", string: &str)],
		sysroot: [("--sysroot",)],
		clean: [("--clean",)],
		sysupgrade: [("--sysupgrade",)],
		refresh: [("--refresh",)],
		asdeps: [("--asdeps",)],
		asexplicit: [("--asexplicit",)],
		ignore: [("--ignore", pkg: &str)],
		ignoregroup: [("--ignoregroup", grp: &str)],
		needed: [("--needed",)],
		overwrite: [("--overwrite", glob: &str)],
		dbpath: [("--dbpath", path: &Path)],
		nodeps: [("--nodeps",)],
		print: [("--print",)],
		root: [("--root", path: &Path)],
		verbose: [("--verbose",)],
		downloadonly: [("--downloadonly",)],
		arch: [("--arch", arch: &str)],
		package: [(,package: &str)],
	}
}

cmd_tool! {
	Makepkg("/usr/bin/makepkg") {
		ignore_arch: [("--ignorearch",)],
		pkgbuild: [("-p", file: &Path)],
		clean_after: [("--clean",)],
		clean_before: [("--cleanbuild",)],
		no_dependencies: [("--nodeps",)],
		no_extract: [("--noextract",)],
		force: [("--force",)],
		generate_integrity: [("--geninteg",)],
		install: [("--install",)],
		log: [("--log",)],
		no_color: [("--nocolor",)],
		no_build: [("--nobuild",)],
		remove_dependencies: [("--rmdeps",)],
		repackage: [("--repackage",)],
		install_dependencies: [("--syncdeps",)],
		source_only_no_download: [("--source",)],
		source_only: [("--allsource",)],
		check: [("--check",)],
		config: [("--config", file: &Path)],
		no_vcs_update: [("--holdver",)],
		key: [("--key", key: &str)],
		no_archive: [("--noarchive",)],
		no_check: [("--nocheck",)],
		no_prepare: [("--noprepare",)],
		no_sign: [("--nosign",)],
		package_list: [("--packagelist",)],
		print_srcinfo: [("--printsrcinfo",)],
		sign: [("--sign",)],
		skip_checksums: [("--skipchecksums",)],
		skip_source_verification: [("--skipinteg",)],
		skip_pgp: [("--skippgpcheck",)],
		verify_source: [("--verifysource",)],
		asdeps: [("--asdeps",)],
		needed: [("--needed",)],
		noconfirm: [("--noconfirm",)],
		noprogress: [("--noprogressbar",)],
	}
}

cmd_tool! {
	Git("/usr/bin/git") {
		clone: [("clone",)],
		pull: [("pull",)],

		repository: [(,url: &str)],
		directory: [(,dir: &Path)],
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_generated_cmd_tools() {
		assert_eq!(
			format!("{:?}", Makepkg::default()
			.ignore_arch()
			.pkgbuild(Path::new("test"))
			.install()
			.install_dependencies()
			.key("test")
			.arg("--")
			.arg("test")
			.into_raw()),
			"\"/usr/bin/makepkg\" \"--ignorearch\" \"-p\" \"test\" \"--install\" \"--syncdeps\" \"--key\" \"test\" \"--\" \"test\""
		)
	}
}

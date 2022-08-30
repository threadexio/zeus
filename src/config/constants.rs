use const_format::formatcp;

macro_rules! from_env {
	($varname:tt, $envvar:tt) => {
		#[allow(dead_code)]
		pub const $varname: &'static str =
			env!($envvar, concat!($envvar, " not set"));
	};
}

#[cfg(debug_assertions)]
const BUILD_TYPE: &'static str = "dbg";

#[cfg(not(debug_assertions))]
const BUILD_TYPE: &'static str = "rls";

pub const VERSION: &'static str =
	formatcp!("{}-{BUILD_TYPE}", env!("VERSION", "VERSION not set"));

from_env!(BUILD_INFO, "BUILD_INFO");

from_env!(NAME, "CARGO_CRATE_NAME");
from_env!(DESCRIPTION, "CARGO_PKG_DESCRIPTION");
from_env!(HOMEPAGE, "CARGO_PKG_HOMEPAGE");
from_env!(REPOSITORY, "CARGO_PKG_REPOSITORY");
from_env!(LICENSE, "CARGO_PKG_LICENSE");
from_env!(AUTHORS, "CARGO_PKG_AUTHORS");

pub const LONG_VERSION: &'static str = formatcp!(
	r#"{} {}

    _oo     Copyright lololol (C) 2022 {}
 >-(_  \
    / _/     This program may be freely distributed under
   / /       the terms of the GNU General Public License v3.0.
  / (
 (   `-.     {}
  `--.._)

	 Defaults:
	   DATA_DIR      | {}
	   BUILDER_NAME  | {}
	   BUILDER_IMAGE | {}
	   BUILD_DIR     | {}
	   AUR_HOST      | {}
	   RUNTIME       | {}
	   RUNTIME_DIR   | {}
"#,
	VERSION,
	BUILD_INFO,
	AUTHORS,
	HOMEPAGE,
	defaults::DATA_DIR,
	defaults::BUILDER_NAME,
	defaults::BUILDER_IMAGE,
	defaults::BUILD_DIR,
	defaults::AUR_HOST,
	defaults::RUNTIME,
	defaults::RUNTIME_DIR,
);

pub mod defaults {
	from_env!(LOG_LEVEL, "DEFAULT_LOG_LEVEL");
	from_env!(DATA_DIR, "DEFAULT_DATA_DIR");
	from_env!(BUILDER_NAME, "DEFAULT_NAME");
	from_env!(BUILDER_IMAGE, "DEFAULT_IMAGE");
	from_env!(BUILD_DIR, "DEFAULT_BUILDDIR");
	from_env!(AUR_HOST, "DEFAULT_AUR_HOST");
	from_env!(RUNTIME, "DEFAULT_RUNTIME");
	from_env!(RUNTIME_DIR, "DEFAULT_RUNTIME_DIR");
}

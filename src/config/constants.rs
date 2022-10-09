use const_format::formatcp;

macro_rules! from_env {
	($varname:ident) => {
		#[allow(dead_code)]
		pub const $varname: &'static str = env!(
			stringify!($varname),
			concat!(stringify!($varname), " not set")
		);
	};
	($varname:ident, $envvar:tt) => {
		#[allow(dead_code)]
		pub const $varname: &'static str =
			env!($envvar, concat!(stringify!($envvar), " not set"));
	};
}

from_env!(NAME, "CARGO_CRATE_NAME");
from_env!(DESCRIPTION, "CARGO_PKG_DESCRIPTION");
from_env!(HOMEPAGE, "CARGO_PKG_HOMEPAGE");
from_env!(REPOSITORY, "CARGO_PKG_REPOSITORY");
from_env!(LICENSE, "CARGO_PKG_LICENSE");
from_env!(AUTHORS, "CARGO_PKG_AUTHORS");

from_env!(VERSION);
from_env!(BUILD_INFO);

from_env!(LOG_LEVEL);
from_env!(DATA_DIR);
from_env!(BUILDER_NAME);
from_env!(BUILDER_IMAGE);
from_env!(BUILD_DIR);
from_env!(AUR_URL);
from_env!(RUNTIME);
from_env!(RUNTIME_DIR);

pub const LONG_VERSION: &'static str = formatcp!(
	r#"{} {}

    _oo     Copyright lololol (C) 2022 {}
 >-(_  \
    / _/     This program may be freely distributed under
   / /       the terms of the GNU General Public License v3.0.
  / (
 (   `-.     {}
  `--.._)

	 Built with:
	   AUR url           | {}
	   Builder image     | {}
	   Builder name      | {}
	   Data directory    | {}
	   Package directory | {}
	   Runtime           | {}
	   Runtime directory | {}
"#,
	VERSION,
	BUILD_INFO,
	AUTHORS,
	HOMEPAGE,
	AUR_URL,
	BUILDER_IMAGE,
	BUILDER_NAME,
	DATA_DIR,
	BUILD_DIR,
	RUNTIME,
	RUNTIME_DIR,
);

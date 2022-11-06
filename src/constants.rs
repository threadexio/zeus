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
	($varname:ident, $envvar:tt = $default:expr) => {
		#[allow(dead_code)]
		pub const $varname: &'static str = match option_env!($envvar)
		{
			Some(v) => v,
			None => $default,
		};
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
from_env!(BUILD_TYPE, "BUILD_TYPE" = "unknown");

from_env!(LOG_LEVEL);
from_env!(DATA_DIR);
from_env!(BUILDER_NAME);
from_env!(BUILDER_IMAGE);
from_env!(BUILD_DIR);
from_env!(AUR_URL);
from_env!(RUNTIME);
from_env!(RUNTIME_DIR);
from_env!(LIB_DIR);

pub const LONG_VERSION: &str = formatcp!(
	r#"{VERSION}-{BUILD_TYPE} {BUILD_INFO}

    _oo     Copyright lololol (C) 2022 {AUTHORS}
 >-(_  \
    / _/     This program may be freely distributed under
   / /       the terms of the GNU General Public License v3.0.
  / (
 (   `-.     {HOMEPAGE}
  `--.._)

	 Built with:
	   AUR url           │ {AUR_URL}
	   Builder image     │ {BUILDER_IMAGE}
	   Builder name      │ {BUILDER_NAME}
	   Data directory    │ {DATA_DIR}
	   Library directory │ {LIB_DIR}
	   Package directory │ {BUILD_DIR}
	   Runtime           │ {RUNTIME}
	   Runtime directory │ {RUNTIME_DIR}
"#
);

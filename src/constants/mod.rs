#![allow(dead_code)]

use const_format::formatcp;

macro_rules! from_env {
	($var:ident) => {
		//from_env!($var, concat!("ZEUS_", stringify!($var)));
		from_env!($var, stringify!($var));
	};
	($var:ident, $env:expr) => {
		pub const $var: &'static str =
			env!($env, concat!($env, " not set"));
	};
	($var:ident = $default:expr) => {
		pub const $varname: &'static str = match option_env!($var) {
			Some(v) => v,
			None => $default,
		};
	};
}

from_env!(NAME, "CARGO_PKG_NAME");
from_env!(DESCRIPTION, "CARGO_PKG_DESCRIPTION");
from_env!(HOMEPAGE, "CARGO_PKG_HOMEPAGE");
from_env!(REPOSITORY, "CARGO_PKG_REPOSITORY");
from_env!(LICENSE, "CARGO_PKG_LICENSE");
from_env!(AUTHORS, "CARGO_PKG_AUTHORS");

from_env!(VERSION);
from_env!(BUILD_INFO);

from_env!(CONFIG_DIR);
from_env!(BUILD_DIR);

from_env!(PREFIX);

pub const BIN_DIR: &str = formatcp!("{PREFIX}/bin");
pub const LIB_DIR: &str = formatcp!("{PREFIX}/lib/zeus");
pub const DATA_DIR: &str = formatcp!("{PREFIX}/share/zeus");

from_env!(BUILDER_NAME);
from_env!(BUILDER_IMAGE);

from_env!(LOG_LEVEL);
from_env!(AUR_URL);
from_env!(RUNTIME);

pub const AUR_IDENTITY: &str = formatcp!("{NAME}/{VERSION}");
pub const CONFIG_FILE: &str = formatcp!("{}/zeus.toml", CONFIG_DIR);

pub const LONG_VERSION: &str = formatcp!(
	r#"{VERSION} {BUILD_INFO}

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
  Runtime           │ {RUNTIME}
  Config directory  │ {CONFIG_DIR}
  Data directory    │ {DATA_DIR}
  Library directory │ {LIB_DIR}
  Package directory │ {BUILD_DIR}
"#,
);

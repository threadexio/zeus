use crate::aur::Aur;

use serde::{Deserialize, Serialize};

use const_format::formatcp;

use std::collections::HashSet;

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

from_env!(NAME, "CARGO_CRATE_NAME");
from_env!(DESCRIPTION, "CARGO_PKG_DESCRIPTION");
from_env!(HOMEPAGE, "CARGO_PKG_HOMEPAGE");
from_env!(REPOSITORY, "CARGO_PKG_REPOSITORY");
from_env!(LICENSE, "CARGO_PKG_LICENSE");
from_env!(AUTHORS, "CARGO_PKG_AUTHORS");

#[allow(dead_code)]
pub mod defaults {
	from_env!(DATA_DIR, "DEFAULT_DATA_DIR");
	from_env!(BUILDER_NAME, "DEFAULT_NAME");
	from_env!(BUILDER_IMAGE, "DEFAULT_IMAGE");
	from_env!(BUILD_DIR, "DEFAULT_BUILDDIR");
	from_env!(AUR_HOST, "DEFAULT_AUR_HOST");
	from_env!(RUNTIME, "DEFAULT_RUNTIME");
	from_env!(RUNTIME_DIR, "DEFAULT_RUNTIME_DIR");
}

// Operations that are handled inside the machine
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation {
	Sync,
	Remove,
	Build,
	Query,
	Completions,
	Runtime,
	None,
}

impl Default for Operation {
	fn default() -> Self {
		Self::None
	}
}

impl From<&str> for Operation {
	fn from(s: &str) -> Self {
		use Operation::*;
		match s {
			"sync" => Sync,
			"remove" => Remove,
			"build" => Build,
			"query" => Query,
			"runtime" => Runtime,
			"completions" => Completions,
			_ => Default::default(),
		}
	}
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppConfig {
	pub operation: Operation,

	/// Should we display debug logs?
	pub debug: bool,

	pub force: bool,

	/// Instance to communicate with the AUR RPC interface
	pub aur: Aur,

	/// Build directory for packages
	pub build_dir: String,

	/// Name of the runtime to load
	pub runtime: String,

	/// Directory to search for runtimes
	pub runtime_dir: String,

	// Sync
	pub upgrade: bool,
	pub build_args: Vec<String>,

	// Machine
	/// Machine name
	pub machine: String,

	/// Machine image name
	pub image: String,

	/// Packages for an operation
	pub packages: HashSet<String>,

	/// Keywords for the query operation
	pub keywords: HashSet<String>,
}

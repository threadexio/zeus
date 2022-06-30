use crate::aur::Aur;

use serde::{Deserialize, Serialize};

use const_format::formatcp;

use std::collections::HashSet;

#[allow(dead_code)]
pub const PROGRAM_NAME: &'static str = "zeus";

#[allow(dead_code)]
pub const PROGRAM_DESC: &'static str = env!("CARGO_PKG_DESCRIPTION");

#[cfg(debug_assertions)]
const BUILD_TYPE: &'static str = "dbg";

#[cfg(not(debug_assertions))]
const BUILD_TYPE: &'static str = "rls";

pub const PROGRAM_VERSION: &'static str =
	formatcp!("{}-{BUILD_TYPE}", env!("VERSION", "VERSION not set"));

#[allow(dead_code)]
pub const DATA_DIR: &'static str =
	env!("DEFAULT_DATA_DIR", "DEFAULT_DATA_DIR not set");

// Operations that are handled inside the machine
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation {
	None,
	Sync,
	Remove,
}

impl Default for Operation {
	fn default() -> Self {
		Self::None
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

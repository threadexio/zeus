use crate::aur::Aur;

use serde::{Deserialize, Serialize};

use const_format::formatcp;

use default_env::default_env;

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
	formatcp!("{}-{BUILD_TYPE}", default_env!("VERSION", "unknown"));

// Operations that are handled inside the container
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
	// Global
	pub operation: Operation,
	pub debug: bool,
	pub force: bool,
	pub aur: Aur,
	pub builddir: String,

	// Sync
	pub upgrade: bool,
	pub buildargs: Vec<String>,

	// Build
	pub archive: String,
	pub dockerfile: String,
	pub image: String,

	// Remove
	pub remove: bool,

	// Sync + Remove + Build
	pub name: String,

	// Sync + Remove
	pub packages: HashSet<String>,

	// Query
	pub keywords: Vec<String>,
}

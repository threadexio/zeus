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

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppConfig {
	// Global
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

	// Remove
	pub remove: bool,

	// Sync + Build
	pub image: String,
	pub name: String,

	// Sync + Remove
	pub packages: HashSet<String>,

	// Query
	pub keywords: Vec<String>,
}

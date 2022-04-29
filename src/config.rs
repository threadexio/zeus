use serde::{Deserialize, Serialize};

use const_format::formatcp;
use default_env::default_env;

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
	pub verbose: bool,
	pub force: bool,

	pub upgrade: bool,
	pub buildargs: Vec<String>,
	pub builddir: String,
	pub packages: Vec<String>,

	pub archive: String,
	pub dockerfile: String,
	pub image: String,
	pub name: String,
}

use serde::{Deserialize, Serialize};

use crate::aur::Package;
use crate::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderPackage {
	pub package: Package,
	pub files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
	Config(Config),
	PackageBuilt(BuilderPackage),
	Success,
	Failure(String),
}

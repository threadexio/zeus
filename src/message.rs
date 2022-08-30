use serde::{Deserialize, Serialize};

use crate::aur::Package;
use crate::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageMeta {
	pub package: Package,
	pub files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
	Config(Config),
	PackageMeta(PackageMeta),
	Success,
	Failure(String),
}

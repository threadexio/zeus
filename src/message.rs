use serde::{Deserialize, Serialize};

use crate::aur::Package;
use crate::config::AppConfig;

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
	Config(AppConfig),
	Success(Vec<Package>),
	Failure(String),
}

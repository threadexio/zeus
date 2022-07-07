use serde::{Deserialize, Serialize};

use crate::config::AppConfig;

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
	Config(AppConfig),
	Success(Vec<String>),
	Failure(String),
}

unsafe impl Sync for Message {}

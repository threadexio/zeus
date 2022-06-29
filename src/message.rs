use serde::{Deserialize, Serialize};

use crate::config::AppConfig;

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
	Config(AppConfig),
	Done,
}

unsafe impl Sync for Message {}

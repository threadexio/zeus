use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use zeus_types::{GlobalConfig, RemoveConfig, SyncConfig};

mod client;
pub use client::Client;

mod listener;
pub use listener::Listener;

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
	Init(GlobalConfig),
	Sync(SyncConfig),
	Remove(RemoveConfig),
	Response(Response),
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Response {
	pub packages: Vec<String>,
	pub files: Vec<PathBuf>,
}

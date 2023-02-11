#![allow(dead_code)]

use ::std::path::PathBuf;

use serde::{Deserialize, Serialize};

mod client;
pub use client::Client;

mod listener;
pub use listener::Listener;

use crate::config::*;

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

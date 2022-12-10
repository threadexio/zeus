// TODO: Add module docs
#![allow(dead_code)]

mod error;

mod client;
mod listener;

use ::std::path::PathBuf;

use crate::config::*;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Message {
	Init(GlobalOptions),
	Sync(SyncOptions),
	Remove(RemoveOptions),
	Response(Response),
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Response {
	pub packages: Vec<String>,
	pub files: Vec<PathBuf>,
}

pub use client::Client;
pub use listener::Listener;

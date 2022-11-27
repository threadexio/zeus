#![allow(dead_code)]

mod error;

mod client;
mod listener;

use crate::config::Config;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Message {
	BuilderInit { config: Config },
	Response { packages: Vec<String> },
}

pub use client::Client;
pub use listener::Listener;

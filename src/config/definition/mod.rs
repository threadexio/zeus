pub(self) mod prelude {
	pub use std::path::Path;
	pub use std::path::PathBuf;

	pub use anyhow::{Context, Result};
	pub use clap::{
		value_parser, Arg, ArgAction, ArgMatches, Command, ValueHint,
	};
	pub use serde::{Deserialize, Serialize};
	pub use toml::Value;

	pub(crate) use super::{
		super::macros::config_path, super::traits::*, super::types::*,
	};

	pub(crate) use crate::constants;
}

mod global;
pub use global::GlobalConfig;

mod sync;
pub use sync::SyncConfig;

mod remove;
pub use remove::RemoveConfig;

mod build;
pub use build::BuildConfig;

mod query;
pub use query::QueryConfig;

mod runtime;
pub use runtime::RuntimeConfig;

mod completions;
pub use completions::CompletionsConfig;

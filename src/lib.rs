#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]

mod aur;
mod config;
mod constants;

pub mod error;
pub mod log;

mod machine;
pub use machine::*;

pub mod prelude {
	// Error handling
	pub use crate::error::*;

	// Interfaces
	pub use crate::config::GlobalOptions;
	pub use crate::{IRuntime, Runtime};

	// Logging macros
	pub use crate::{debug, error, info, warn};

	pub use crate::declare_runtime;
}

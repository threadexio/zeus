//! How to implement a basic out-of-tree runtime:
//!
//! # Step 1:
//! Create a rust library
//! ```bash
//! $ cargo new --lib my_runtime
//! ```
//!
//! # Step 2:
//! Open `Cargo.toml` and add:
//! ```toml
//! [lib]
//! name = "rt_my_runtime"
//! crate-type = ["cdylib"]
//!
//! [dependencies.zeus]
//! path = "<path to the local zeus repo>"
//! ```
//! > **NOTE**: The `rt_` prefix in `lib.name` is necessary.
//!
//! Yes, you have to have a copy of the `zeus` repository
//! available locally.
//!
//! # Step 3:
//! Open `src/lib.rs` and delete the default contents, then you need to:
//!  - Define your runtime as a `struct`
//!  - Implement `zeus::IRuntime` on it
//!  - Implement a constructor on it (eg `new()` or simply `#[derive(Default)]`)
//!  - Register it with the `zeus::runtime!()` macro
//!
//! # Example
//! ```rust,ignore
//! // lib.rs
//!
//! use zeus::*;
//!
//! #[derive(Default)]
//! struct MyRuntime;
//!
//! impl IRuntime for MyRuntime {
//!     // ...
//! }
//!
//! runtime!(MyRuntime::default);
//! // or if you implemented `new()`
//! //runtime!(MyRuntime::new);
//! ```
//!
//! For a complete working example see: [`runtimes/zeus_rt_docker`](../src/rt_docker/lib.rs.html)
#![deny(clippy::correctness)]
#![warn(
	clippy::style,
	clippy::complexity,
	clippy::perf,
	clippy::unwrap_used,
	clippy::expect_used
)]

mod aur;
mod constants;

#[doc(hidden)]
pub mod log;

mod config;
pub use config::GlobalOptions;

mod error;
pub use error::*;

mod runtime;
pub use runtime::IRuntime;

mod interface;
mod loader;

pub use interface::*;
pub use loader::_private;

pub use loader::Runtime;

pub(crate) const RUSTC_VERSION: &str = env!("RUSTC_VERSION");

use std::path::Path;

use anyhow::{bail, Context, Result};
use libloading::{Library, Symbol};

use super::interface::IRuntime;

pub struct Runtime {
	runtime: Box<dyn IRuntime>,
	library: Library,
}

impl std::ops::Deref for Runtime {
	type Target = dyn IRuntime;

	fn deref(&self) -> &Self::Target {
		self.runtime.as_ref()
	}
}

impl std::ops::DerefMut for Runtime {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.runtime.as_mut()
	}
}

impl Drop for Runtime {
	fn drop(&mut self) {
		self.runtime.exit();
	}
}

unsafe impl Send for Runtime {}
unsafe impl Sync for Runtime {}

impl Runtime {
	pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
		unsafe {
			let library = Library::new(path.as_ref())?;

			let runtime_meta: Symbol<&_private::RuntimeMeta> =
				library
					.get(_private::RUNTIME_META_SYMBOL_NAME)
					.context("failed to find runtime data symbol")?;

			if runtime_meta.version != _private::RUNTIME_VERSION {
				bail!("incompatible runtime version")
			}

			let runtime = (runtime_meta.constructor)();

			Ok(Self { library, runtime })
		}
	}
}

/// Define a runtime.
///
/// Syntax:
/// ```rust,ignore
/// zeus::runtime!(/* runtime constructor */);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! runtime {
	($constructor:path) => {
		#[doc(hidden)]
		#[no_mangle]
		pub static _RUNTIME: $crate::runtime::_private::RuntimeMeta =
			$crate::runtime::_private::RuntimeMeta {
				constructor: || Box::new($constructor()),
				version: $crate::runtime::_private::RUNTIME_VERSION,
			};
	};
}
pub use runtime;

#[doc(hidden)]
pub mod _private {
	use super::*;

	pub(crate) const RUNTIME_META_SYMBOL_NAME: &[u8] = b"_RUNTIME";

	pub const RUNTIME_VERSION: u32 = 0;

	pub struct RuntimeMeta {
		pub constructor: fn() -> Box<dyn IRuntime>,
		pub version: u32,
	}
}

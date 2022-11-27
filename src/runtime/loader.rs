use std::path::Path;

use libloading::{Library, Symbol};

use super::error::*;
use super::IRuntime;

pub struct Runtime {
	runtime: Box<dyn IRuntime>,

	library: Library,
}

impl Runtime {
	pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
		unsafe {
			let library = Library::new(path.as_ref())?;

			let constructor: Symbol<
				unsafe fn() -> *mut dyn IRuntime,
			> = library.get(b"_runtime")?;

			let runtime = Box::from_raw(constructor());

			Ok(Self { library, runtime })
		}
	}
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

unsafe impl Send for Runtime {}
unsafe impl Sync for Runtime {}

/// Define a runtime.
///
/// Syntax:
/// ```rust,ignore
/// zeus::runtime!(/* runtime constructor */);
/// ```
#[macro_export]
macro_rules! runtime {
	($constructor:path) => {
		#[doc(hidden)]
		#[no_mangle]
		pub extern "C" fn _runtime() -> *mut dyn $crate::IRuntime {
			Box::into_raw(Box::new($constructor()))
		}
	};
}

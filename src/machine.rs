//! Each runtime runs in a shared environment with the rest of the program,
//! this means that whatever process-global data is changed by the runtime
//! should be restored back to their initial state. This for example includes
//! the CWD. When the runtime's `init()` is ran, the CWD is set to the program's
//! data directory where it can store any persistent internal data that is not
//! handled by an external daemon. The runtime is only allowed to access resources
//! outside that path the current user has access to. It can't just read `/etc/shadow`,
//! unless of course `zeus` is running as root. This can be mitigated by modifying
//! the Apparmor rules to allow or disallow access. This requires the runtime developers
//! to install their policy inside /etc/apparmor.d/zeus.d.

use std::path::Path;

pub use crate::config::GlobalOptions;
use crate::error::*;

use libloading::{Library, Symbol};

pub struct Runtime {
	// As of rust 1.61.0, field order matters, and we have to drop `runtime` first, otherwise this just segfaults.
	// This has caused me great pain.
	// Do NOT change the order of these fields.
	runtime: Box<dyn IRuntime>,
	#[allow(dead_code)]
	library: Library,
}

#[allow(dead_code)]
impl Runtime {
	pub(crate) fn load(
		path: &Path,
		opts: &GlobalOptions,
	) -> Result<Self> {
		unsafe {
			let library = Library::new(path)?;

			let constructor: Symbol<
				unsafe fn() -> *mut dyn IRuntime,
			> = library
				.get(b"_runtime_create")
				.context("Could not find runtime constructor")?;

			let mut runtime = Box::from_raw(constructor());

			if runtime.rt_api_version() != Self::RT_API_VERSION {
				return Err(Error::new(format!(
					"Incompatible runtime ({}), supported ({})",
					runtime.rt_api_version(),
					Self::RT_API_VERSION
				)));
			}

			runtime
				.init(opts)
				.context("Error during runtime initialization")?;

			Ok(Self { library, runtime })
		}
	}

	pub(crate) fn unload(self) {
		drop(self)
	}

	/// Increasing this number means there has been a breaking change in the API.
	/// Removing or changing method signatures is a breaking change.
	pub const RT_API_VERSION: u32 = 4;
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

/// A trait specifying a common interface for all machine runtime drivers.
pub trait IRuntime {
	/// Runtime driver name
	fn name(&self) -> &'static str;
	/// Runtime driver version
	fn version(&self) -> &'static str;

	/// A simplistic way to signal breaking changes in the API for runtimes.
	///
	/// If `runtime.rt_api_version()` != `constants::RT_API_VERSION`,
	/// then the runtime will be considered incompatible and not load.
	fn rt_api_version(&self) -> u32;

	/// This will be ran on driver load.
	///
	/// Returning an Err variant here will exit the program immediately reporting the error to the user.
	fn init(&mut self, config: &GlobalOptions) -> Result<()>;

	/// This will be ran on driver unload.
	fn exit(&mut self);

	/// List images returning a vector containing their names.
	fn list_images(&self) -> Result<Vec<String>>;

	/// Create or update an image.
	fn make_image(&mut self, image_name: &str) -> Result<()>;

	/// Delete an image.
	///
	/// If:
	/// 	- the image does NOT exist
	/// 	- there are machines using the image
	///
	/// Then:
	///
	/// An error should be returned.
	fn delete_image(&mut self, image_name: &str) -> Result<()>;

	/// List machines returning a vector containing their names.
	fn list_machines(&self) -> Result<Vec<String>>;

	/// Create a machine and apply the necessary configuration.
	///
	/// If:
	/// 	- the machine already exists
	///
	/// Then:
	///
	/// An error should be returned.
	fn create_machine(
		&mut self,
		machine_name: &str,
		image_name: &str,
		config: &GlobalOptions,
	) -> Result<()>;

	/// Start a machine and attach it to the terminal. The runtime is responsible for having
	/// forwarded the communication socket to the machine.
	///
	/// If:
	/// 	- the machine does NOT exist
	///
	/// Then:
	///
	/// An error should be returned.
	fn start_machine(&mut self, machine_name: &str) -> Result<()>;

	/// Stop a machine.
	///
	/// If:
	/// 	- the machine does NOT exist
	///
	/// Then:
	///
	/// An error should be returned.
	fn stop_machine(&mut self, machine_name: &str) -> Result<()>;

	/// Delete a machine completely.
	///
	/// If:
	/// 	- the machine does NOT exist
	///
	/// Then:
	///
	/// An error should be returned.
	fn delete_machine(&mut self, machine_name: &str) -> Result<()>;
}

#[macro_export]
macro_rules! declare_runtime {
	($plugin:ty, $constructor:path) => {
		#[no_mangle]
		pub extern "C" fn _runtime_create(
		) -> *mut dyn $crate::IRuntime {
			let constructor: fn() -> $plugin = $constructor;
			let boxed = Box::new(constructor());
			Box::into_raw(boxed)
		}
	};
}

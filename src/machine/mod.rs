//! Each runtime runs in a shared environment with the rest of the program,
//! this means that whatever process-global data is changed by the runtime
//! should be restored back to their initial state. This for example includes
//! the CWD. When the runtime's `init()` is ran, the CWD is set to the program's
//! data directory where it can store any persistent internal data that is not
//! handled by an external daemon. The runtime is only allowed to access resources
//! outside that path the current user has access to. It can't just read `/etc/shadow`,
//! unless of course `zeus` is running as root. This can be mitigated by modifying
//! the Apparmor rules to allow or disallow access. This requires the runtime
//! developers to work together with the maintainers and developers of `zeus` or
//! issue a patch for the Apparmor profile that each user has to apply.

pub use crate::config::AppConfig;
pub use std::io::{Read, Write};

pub mod constants {
	use super::*;

	/// Increasing this number means there has been a breaking change in the API.
	/// Removing or changing method signatures is a breaking change.
	pub const MAX_SUPPORTED_RT_API_VERSION: u32 = 1;

	// These should never really be changed
	pub const RUNTIME_CONSTRUCTOR_SYMBOL_NAME: &'static str =
		"_runtime_create";
	pub type RuntimeConstructorSymbol = unsafe fn() -> *mut Runtime;
}

pub(crate) mod manager;

pub type Error = String;
pub type Result<T> = std::result::Result<T, Error>;

/// A trait specifying a common interface for all machine runtime drivers.
pub type Runtime = dyn IRuntime;
pub type BoxedRuntime = Box<Runtime>;
pub trait IRuntime {
	/// Runtime driver name
	fn name(&self) -> &'static str;
	/// Runtime driver version
	fn version(&self) -> &'static str;

	/// A simplistic way to signal breaking changes in the API for runtimes.
	///
	/// If `runtime.rt_api_version()` != `constants::SUPPORTED_RT_API_VERSION`,
	/// then the runtime will be considered incompatible and not load.
	fn rt_api_version(&self) -> u32;

	/// This will be ran on driver load.
	///
	/// Returning an Err variant here will exit the program immediately reporting the error to the user.
	fn init(&mut self) -> Result<()>;

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
		config: &AppConfig,
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
		) -> *mut $crate::machine::Runtime {
			let constructor: fn() -> $plugin = $constructor;
			let boxed = Box::new(constructor());
			Box::into_raw(boxed)
		}
	};
}

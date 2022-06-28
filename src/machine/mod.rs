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

	/// Increase this number when making a breaking change in the runtime API bellow.
	/// Removing or changing method signatures is considered a breaking change.
	pub static SUPPORTED_RT_API_VERSION: u32 = 1;

	// These should never really be changed
	pub static RUNTIME_CONSTRUCTOR_SYMBOL_NAME: &'static str =
		"_runtime_create";
	pub type RuntimeConstructorSymbol = unsafe fn() -> *mut Runtime;
}

pub(crate) mod manager;

pub type Error = String;
pub type Result<T> = std::result::Result<T, Error>;

pub type Image = dyn IImage;
pub type BoxedImage = Box<Image>;
pub trait IImage {
	/// Get the ID of the image
	fn id(&self) -> &str;
	/// Get the name of the image
	fn name(&self) -> &str;
}

pub type Machine = dyn IMachine;
pub type BoxedMachine = Box<Machine>;
pub trait IMachine {
	/// Get the ID of the machine
	fn id(&self) -> &str;
	/// Get the name of the machine
	fn name(&self) -> &str;
	/// Get the ID of the image used for the machine
	fn image(&self) -> &str;
}

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
	/// `runtime.rt_api_version()` != `constants::SUPPORTED_RT_API_VERSION`, then the runtime will be considered incompatible and not load.
	fn rt_api_version(&self) -> u32;

	/// This will be ran on driver load.
	///
	/// Returning an Err variant here will exit the program immediately reporting the error to the user.
	fn init(&mut self) -> Result<()>;

	/// This will be ran on driver unload.
	fn exit(&mut self);

	/// List images returning a vector containing their names.
	fn list_images(&self) -> Result<Vec<BoxedImage>>;

	/// Create an image with all the necessary configuration
	/// If the image already exists then it should be updated.
	fn create_image(
		&mut self,
		image_name: &str,
	) -> Result<BoxedImage>;

	/// Update an image. If the image does not exist, it should be created.
	fn update_image(&mut self, image: &Image) -> Result<()>;

	/// Delete an image. If there are any machines using that image, they should all be removed.
	fn delete_image(&mut self, image: BoxedImage) -> Result<()>;

	/// List machines returning a vector containing their names
	fn list_machines(&self) -> Result<Vec<BoxedMachine>>;

	/// Create a machine and apply the necessary configuration. If the machine already exists, an Ok variant should be returned.
	fn create_machine(
		&mut self,
		machine_name: &str,
		image: &Image,
		config: &AppConfig,
	) -> Result<BoxedMachine>;

	/// Start a machine. If the machine does not exist, an error should be returned.
	fn start_machine(&mut self, machine: &Machine) -> Result<()>;

	/// Stop a machine. If the machine does not exist, an Ok variant should be returned.
	fn stop_machine(&mut self, machine: &Machine) -> Result<()>;

	/// Attach to a machine and return the attached stdin and stdout streams.
	/// If the machine does not exist, an error should be returned.
	fn attach_machine(&mut self, machine: &Machine) -> Result<()>;

	/// Execute command in a machine and get its exit code.
	/// If the machine does not exist, an error should be returned.
	fn execute_command(
		&mut self,
		machine: &Machine,
		command: &str,
	) -> Result<i32>;

	/// Delete a machine completely. If the machine does not exist, an Ok variant should be returned.
	fn delete_machine(&mut self, machine: BoxedMachine)
		-> Result<()>;
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

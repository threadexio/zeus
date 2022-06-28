pub mod constants {
	use super::*;

	/// Increase this number when making a breaking change in the runtime API bellow.
	/// Adding, removing or changing method signatures is considered a breaking change.
	pub static MIN_SUPPORTED_RT_API_VERSION: u32 = 1;

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
	fn name(&self) -> String;
}

pub type Machine = dyn IMachine;
pub type BoxedMachine = Box<Machine>;
pub trait IMachine {
	fn name(&self) -> String;
	fn image(&self) -> BoxedImage;
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
	/// `runtime.rt_api_version()` < `constants::MIN_SUPPORTED_RT_API_VERSION`, then the runtime will be considered incompatible and not load
	fn rt_api_version(&self) -> u32;

	/// This will be ran on driver load
	///
	/// Returning an Err variant here will exit the program immediately reporting the error to the user.
	fn init(&mut self) -> Result<()>;

	/// This will be ran on driver unload
	fn exit(&mut self);

	// Images

	/// List images returning a vector containing their names
	fn list_images(&self) -> Result<Vec<String>>;

	fn get_image(&self, image_name: &str) -> Result<BoxedImage>;

	/// Create an image with all the necessary configuration
	fn create_image(&mut self, image_name: &str) -> Result<()>;

	/// Update an image
	fn update_image(&mut self, image_name: &str) -> Result<()>;

	/// Delete an image and all machines using it
	fn delete_image(&mut self, image_name: &str) -> Result<()>;

	// Machines

	/// List machines returning a vector containing their names
	fn list_machines(&self) -> Result<Vec<String>>;

	fn get_machine(&self, machine_name: &str)
		-> Result<BoxedMachine>;

	/// Create a machine and apply the necessary configuration
	fn create_machine(
		&mut self,
		machine_name: &str,
		image_name: &str,
	) -> Result<()>;

	/// Stop a machine gracefully or not
	fn stop_machine(&mut self, machine_name: &str) -> Result<()>;

	/// Delete a machine completely
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

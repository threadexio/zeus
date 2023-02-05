pub use crate::config::GlobalConfig;
use anyhow::Result;

/// A trait specifying a common interface for all machine runtime drivers.
pub trait IRuntime {
	/// Runtime driver name
	fn name(&self) -> &'static str;
	/// Runtime driver version
	fn version(&self) -> &'static str;

	/// This will be ran on driver load.
	///
	/// Returning an Err variant here will exit the program immediately reporting the error to the user.
	fn init(&mut self, config: &GlobalConfig) -> Result<()>;

	/// This will be ran on driver unload.
	fn exit(&mut self);

	/// List images returning a vector containing their names.
	fn list_images(&self) -> Result<Vec<String>>;

	/// Create or update an image.
	fn make_image(&mut self, image_name: &str) -> Result<()>;

	/// Delete an image.
	///
	/// If:
	///     - the image does NOT exist
	///     - there are machines using the image
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
	///     - the machine already exists
	///
	/// Then:
	///
	/// An error should be returned.
	fn create_machine(
		&mut self,
		machine_name: &str,
		image_name: &str,
		config: &GlobalConfig,
	) -> Result<()>;

	/// Start a machine and attach it to the terminal. The runtime is responsible for having
	/// forwarded the communication socket to the machine.
	///
	/// If:
	///     - the machine does NOT exist
	///
	/// Then:
	///
	/// An error should be returned.
	fn start_machine(&mut self, machine_name: &str) -> Result<()>;

	/// Stop a machine.
	///
	/// If:
	///     - the machine does NOT exist
	///
	/// Then:
	///
	/// An error should be returned.
	fn stop_machine(&mut self, machine_name: &str) -> Result<()>;

	/// Delete a machine completely.
	///
	/// If:
	///     - the machine does NOT exist
	///
	/// Then:
	///
	/// An error should be returned.
	fn delete_machine(&mut self, machine_name: &str) -> Result<()>;
}

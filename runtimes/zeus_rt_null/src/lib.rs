use zeus::*;

#[derive(Default)]
struct NullRuntime;

impl IRuntime for NullRuntime {
	fn name(&self) -> &'static str {
		"null"
	}

	fn version(&self) -> &'static str {
		env!("CARGO_PKG_VERSION", "must be built with cargo")
	}

	fn init(&mut self, _: &GlobalOptions) -> Result<()> {
		info!("Hello world!");

		warning!(
			"This runtime does nothing! It is only used for testing."
		);

		Ok(())
	}

	fn exit(&mut self) {
		info!("Goodbye cruel world!");
	}

	fn list_images(&self) -> Result<Vec<String>> {
		info!("Runtime::list_images()");
		Ok(vec![])
	}

	fn make_image(&mut self, _image_name: &str) -> Result<()> {
		info!("Runtime::make_image()");
		Ok(())
	}

	fn delete_image(&mut self, _image_name: &str) -> Result<()> {
		info!("Runtime::delete_image()");
		Ok(())
	}

	fn list_machines(&self) -> Result<Vec<String>> {
		info!("Runtime::list_machines()");
		Ok(vec![])
	}

	fn create_machine(
		&mut self,
		_machine_name: &str,
		_image_name: &str,
		_config: &GlobalOptions,
	) -> Result<()> {
		info!("Runtime::create_machine()");
		Ok(())
	}

	fn start_machine(&mut self, _machine_name: &str) -> Result<()> {
		info!("Runtime::start_machine()");
		Ok(())
	}

	fn stop_machine(&mut self, _machine_name: &str) -> Result<()> {
		info!("Runtime::stop_machine()");
		Ok(())
	}

	fn delete_machine(&mut self, _machine_name: &str) -> Result<()> {
		info!("Runtime::delete_machine()");
		todo!()
	}
}

runtime!(NullRuntime::default);

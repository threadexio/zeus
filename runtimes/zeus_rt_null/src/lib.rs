use zeus::prelude::*;

#[derive(Default)]
pub struct NullRuntime;

declare_runtime!(NullRuntime, NullRuntime::default);

impl zeus::IRuntime for NullRuntime {
	fn name(&self) -> &'static str {
		"null"
	}

	fn version(&self) -> &'static str {
		env!("CARGO_PKG_VERSION", "must be built with cargo")
	}

	fn rt_api_version(&self) -> u32 {
		Runtime::RT_API_VERSION
	}

	fn init(&mut self, _: &GlobalOptions) -> Result<()> {
		info!("Hello world!");

		Ok(())
	}

	fn exit(&mut self) {
		info!("Goodbye cruel world!");
	}

	fn list_images(&self) -> Result<Vec<String>> {
		Ok(vec![])
	}

	fn make_image(&mut self, _image_name: &str) -> Result<()> {
		Ok(())
	}

	fn delete_image(&mut self, _image_name: &str) -> Result<()> {
		Ok(())
	}

	fn list_machines(&self) -> Result<Vec<String>> {
		Ok(vec![])
	}

	fn create_machine(
		&mut self,
		_machine_name: &str,
		_image_name: &str,
		_config: &GlobalOptions,
	) -> Result<()> {
		Ok(())
	}

	fn start_machine(&mut self, _machine_name: &str) -> Result<()> {
		Ok(())
	}

	fn stop_machine(&mut self, _machine_name: &str) -> Result<()> {
		Ok(())
	}

	fn delete_machine(&mut self, _machine_name: &str) -> Result<()> {
		todo!()
	}
}

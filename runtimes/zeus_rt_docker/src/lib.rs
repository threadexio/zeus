use zeus::*;

#[derive(Default)]
pub struct DockerRuntime;

declare_runtime!(DockerRuntime, DockerRuntime::default);

impl machine::IRuntime for DockerRuntime {
	fn name(&self) -> &'static str {
		"docker"
	}

	fn version(&self) -> &'static str {
		"0.1.0"
	}

	fn rt_api_version(&self) -> u32 {
		1
	}

	fn init(&mut self) -> machine::Result<()> {
		println!("docker runtime loaded!");
		Ok(())
	}

	fn exit(&mut self) {
		println!("docker runtime unloaded!");
	}

	fn list_images(&self) -> machine::Result<Vec<String>> {
		todo!()
	}

	fn get_image(
		&self,
		_: &str,
	) -> machine::Result<machine::BoxedImage> {
		todo!()
	}

	fn create_image(&mut self, _: &str) -> machine::Result<()> {
		todo!()
	}

	fn update_image(&mut self, _: &str) -> machine::Result<()> {
		todo!()
	}

	fn delete_image(&mut self, _: &str) -> machine::Result<()> {
		todo!()
	}

	fn list_machines(&self) -> machine::Result<Vec<String>> {
		todo!()
	}

	fn get_machine(
		&self,
		_: &str,
	) -> machine::Result<machine::BoxedMachine> {
		todo!()
	}

	fn create_machine(
		&mut self,
		_: &str,
		_: &str,
	) -> machine::Result<()> {
		todo!()
	}

	fn stop_machine(&mut self, _: &str) -> machine::Result<()> {
		todo!()
	}

	fn delete_machine(&mut self, _: &str) -> machine::Result<()> {
		todo!()
	}
}

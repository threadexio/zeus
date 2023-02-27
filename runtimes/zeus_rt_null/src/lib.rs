use zeus::runtime::*;

#[derive(Default)]
struct NullRuntime;

impl IRuntime for NullRuntime {
	fn name(&self) -> &'static str {
		"null"
	}

	fn version(&self) -> &'static str {
		env!("CARGO_PKG_VERSION", "must be built with cargo")
	}

	fn init(
		&mut self,
		config: &GlobalConfig,
		term: &mut Terminal,
	) -> Result<()> {
		term.info("Hello world!");

		term.warn(
			"This runtime does nothing! It is only used for testing.",
		);

		term.trace(format!(
			"runtime options = {:#?}",
			config.runtime_opts
		));

		Ok(())
	}

	fn exit(&mut self) {}

	fn create_image(
		&mut self,
		config: &GlobalConfig,
		term: &mut Terminal,
	) -> Result<()> {
		term.debug(format!(
			"create_image(image = `{}`)",
			config.machine_image
		));
		Ok(())
	}

	fn create_machine(
		&mut self,
		config: &GlobalConfig,
		term: &mut Terminal,
	) -> Result<()> {
		term.debug(format!(
			"create_machine(image = `{}`, machine = `{}`)",
			config.machine_image, config.machine_name
		));
		Ok(())
	}

	fn start_machine(
		&mut self,
		config: &GlobalConfig,
		term: &mut Terminal,
	) -> Result<()> {
		term.debug(format!(
			"start_machine(machine = `{}`)",
			config.machine_name
		));
		Ok(())
	}
}

runtime!(NullRuntime::default);

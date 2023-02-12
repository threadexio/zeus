use zeus::{log::macros::*, runtime::*};

#[derive(Default)]
struct NullRuntime;

impl IRuntime for NullRuntime {
	fn name(&self) -> &'static str {
		"null"
	}

	fn version(&self) -> &'static str {
		env!("CARGO_PKG_VERSION", "must be built with cargo")
	}

	fn init(&mut self, config: &GlobalConfig) -> Result<()> {
		set_log_level!(config.log_level);

		info!("Hello world!");

		warning!(
			"This runtime does nothing! It is only used for testing."
		);

		trace!("runtime options = {:#?}", config.runtime_opts);

		Ok(())
	}

	fn exit(&mut self) {
		debug!("Goodbye cruel world!");
	}

	fn create_image(&mut self, config: &GlobalConfig) -> Result<()> {
		debug!("create_image(image = `{}`)", config.machine_image);
		Ok(())
	}

	fn create_machine(
		&mut self,
		config: &GlobalConfig,
	) -> Result<()> {
		debug!(
			"create_machine(image = `{}`, machine = `{}`)",
			config.machine_image, config.machine_name
		);
		Ok(())
	}

	fn start_machine(&mut self, config: &GlobalConfig) -> Result<()> {
		debug!("start_machine(machine = `{}`)", config.machine_name);
		Ok(())
	}
}

runtime!(NullRuntime::default);

use std::env;
use std::path::{Path, PathBuf};
use std::process::{self, Stdio};

use zeus::runtime::*;

struct CustomRuntime {
	script_dir: PathBuf,
}

impl CustomRuntime {
	fn script(&self, script: &str) -> process::Command {
		let mut cmd =
			process::Command::new(self.script_dir.join(script));
		cmd.stdin(Stdio::inherit())
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit());
		cmd
	}

	fn run_script(&self, mut cmd: process::Command) -> Result<()> {
		let output = match cmd.output() {
			Ok(v) => v,
			Err(e) => {
				return Err(Error::new(format!(
					"unable to run script `{}`: {e}",
					cmd.get_program().to_string_lossy()
				)))
			},
		};

		if !output.status.success() {
			return Err(Error::new(format!(
				"script exited with: {}",
				output.status.code().unwrap_or(-1)
			)));
		}

		Ok(())
	}
}

impl Default for CustomRuntime {
	fn default() -> Self {
		Self { script_dir: Path::new("scripts").to_path_buf() }
	}
}

impl IRuntime for CustomRuntime {
	fn name(&self) -> &'static str {
		"custom"
	}

	fn version(&self) -> &'static str {
		env!("CARGO_PKG_VERSION", "must be built with cargo")
	}

	fn init(
		&mut self,
		config: &GlobalConfig,
		_: &mut Terminal,
	) -> Result<()> {
		let opts = &config.runtime_opts;

		if let Some(v) = env::var_os("SCRIPT_DIR")
			.map(|x| Path::new(&x).to_path_buf())
			.or(opts
				.get("ScriptDir")
				.map(|x| Path::new(&x).to_path_buf()))
		{
			self.script_dir = v;
		}

		self.run_script(self.script("init"))?;

		Ok(())
	}

	fn exit(&mut self) {}

	fn create_image(
		&mut self,
		_: &GlobalConfig,
		_: &mut Terminal,
	) -> Result<()> {
		self.run_script(self.script("create_image"))?;
		Ok(())
	}

	fn create_machine(
		&mut self,
		_: &GlobalConfig,
		_: &mut Terminal,
	) -> Result<()> {
		self.run_script(self.script("create_machine"))?;
		Ok(())
	}

	fn start_machine(
		&mut self,
		_: &GlobalConfig,
		_: &mut Terminal,
	) -> Result<()> {
		self.run_script(self.script("create_machine"))?;
		Ok(())
	}
}

runtime!(CustomRuntime::default);

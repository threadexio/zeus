//! This is the zeus runtime driver for Docker.
//! Available user configuration is done by the
//! following environment variables:
//! - `DOCKER_BIN` - This must point to the docker cli tool. (default: `/usr/bin/docker`)
use zeus::*;

use std::env;
use std::ffi::OsString;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::process::{self, Stdio};

const DOCKER_BIN: &str = "/usr/bin/docker";

fn run_docker(mut cmd: process::Command) -> Result<process::Output> {
	let output = cmd.output().with_context(|| {
		let mut c = String::with_capacity(1024);
		c.push_str(&cmd.get_program().to_string_lossy());
		for arg in cmd.get_args() {
			c.push_str(&arg.to_string_lossy());
		}

		format!("Failed to execute: `{c}`")
	})?;

	if !output.status.success() {
		bail!(
			"Docker failed with: {}",
			output.status.code().unwrap_or(-1)
		)
	}

	Ok(output)
}

#[derive(Default)]
pub struct DockerRuntime {
	docker_bin: PathBuf,
}

impl IRuntime for DockerRuntime {
	fn name(&self) -> &'static str {
		"docker"
	}

	fn version(&self) -> &'static str {
		env!("CARGO_PKG_VERSION", "must be built with cargo")
	}

	fn init(&mut self, config: &GlobalConfig) -> Result<()> {
		set_log_level!(config.log_level);

		self.docker_bin = Path::new(
			&env::var_os("DOCKER_BIN")
				.unwrap_or_else(|| OsString::from(DOCKER_BIN)),
		)
		.to_path_buf();

		Ok(())
	}

	fn exit(&mut self) {}

	fn list_images(&self) -> Result<Vec<String>> {
		let mut cmd = process::Command::new(&self.docker_bin);
		cmd.stdout(Stdio::piped())
			.arg("image")
			.arg("ls")
			.args(["--format", "{{.Repository}}"]);
		let output = run_docker(cmd)?;

		let mut images: Vec<String> = Vec::new();

		for line in output.stdout.lines() {
			let image = match line {
				Ok(v) => v,
				Err(e) => {
					warning!("Failed to parse image line: {e}");
					continue;
				},
			};

			images.push(image);
		}

		Ok(images)
	}

	fn make_image(&mut self, image_name: &str) -> Result<()> {
		let build_context = String::from("./");

		let mut cmd = process::Command::new(&self.docker_bin);
		cmd.arg("build")
			.arg("--pull")
			.arg("--rm")
			.args(["-t", image_name])
			.arg("--")
			.arg(build_context);
		run_docker(cmd)?;

		Ok(())
	}

	fn delete_image(&mut self, image_name: &str) -> Result<()> {
		let mut cmd = process::Command::new(&self.docker_bin);
		cmd.arg("image").arg("rm").arg("--").arg(image_name);
		run_docker(cmd)?;

		Ok(())
	}

	fn list_machines(&self) -> Result<Vec<String>> {
		let mut cmd = process::Command::new(&self.docker_bin);
		cmd.stdout(Stdio::piped())
			.arg("container")
			.arg("ls")
			.arg("-a")
			.args(["--format", "{{.Names}}"]);
		let output = run_docker(cmd)?;

		let mut containers: Vec<String> = Vec::new();

		for line in output.stdout.lines() {
			let container = match line {
				Ok(v) => v,
				Err(e) => {
					warning!("Failed to parse container line: {e}");
					continue;
				},
			};

			containers.push(container);
		}

		Ok(containers)
	}

	fn create_machine(
		&mut self,
		machine_name: &str,
		image_name: &str,
		opts: &GlobalConfig,
	) -> Result<()> {
		let mut cmd = process::Command::new(&self.docker_bin);
		cmd.arg("container")
			.arg("create")
			.arg("-i")
			.arg("-t")
			.args(["--name", machine_name])
			.args([
				"-v",
				"/var/cache/pacman/pkg:/var/cache/pacman/pkg:rw",
			])
			.args([
				"-v",
				&format!(
					"{}:/build:rw",
					opts.build_dir.to_string_lossy()
				),
			])
			.arg("--cap-drop=all")
			.arg("--cap-add=CAP_SETUID")
			.arg("--cap-add=CAP_SETGID")
			.arg("--cap-add=CAP_SYS_CHROOT")
			.arg("--")
			.arg(image_name);
		run_docker(cmd)?;

		Ok(())
	}

	fn start_machine(&mut self, machine_name: &str) -> Result<()> {
		let mut cmd = process::Command::new(&self.docker_bin);
		cmd.arg("container")
			.arg("start")
			.arg("-a")
			.arg("-i")
			.arg("--")
			.arg(machine_name);
		run_docker(cmd)?;

		Ok(())
	}

	fn stop_machine(&mut self, machine_name: &str) -> Result<()> {
		let mut cmd = process::Command::new(&self.docker_bin);
		cmd.arg("container").arg("kill").arg("--").arg(machine_name);
		run_docker(cmd)?;

		Ok(())
	}

	fn delete_machine(&mut self, machine_name: &str) -> Result<()> {
		let mut cmd = process::Command::new(&self.docker_bin);
		cmd.arg("container")
			.arg("rm")
			.arg("-f")
			.arg("-v")
			.arg("--")
			.arg(machine_name);
		run_docker(cmd)?;

		Ok(())
	}
}

runtime!(DockerRuntime::default);

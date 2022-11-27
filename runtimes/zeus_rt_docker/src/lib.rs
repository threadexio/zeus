//! This is the zeus runtime driver for Docker.
//! Available user configuration is done by the
//! following environment variables:
//! - `DOCKER_BIN` - This must point to the docker cli tool. (default: `/usr/bin/docker`)
use zeus::*;

use std::env;
use std::io::BufRead;
use std::process;

mod models;

macro_rules! check_exit {
	($output:expr) => {
		if !$output.status.success() {
			return Err(Error::new_with_context(
				format!(
					"{}",
					String::from_utf8_lossy(&$output.stderr[..])
				),
				"Docker failed",
			));
		}
	};
}

#[derive(Default)]
pub struct DockerRuntime {
	docker_bin: String,
}

runtime!(DockerRuntime::default);

impl IRuntime for DockerRuntime {
	fn name(&self) -> &'static str {
		"docker"
	}

	fn version(&self) -> &'static str {
		env!("CARGO_PKG_VERSION", "must be built with cargo")
	}

	fn init(&mut self, _: &GlobalOptions) -> Result<()> {
		self.docker_bin = env::var("DOCKER_BIN")
			.unwrap_or_else(|_| String::from("/usr/bin/docker"));

		Ok(())
	}

	fn exit(&mut self) {}

	fn list_images(&self) -> Result<Vec<String>> {
		let child = process::Command::new(&self.docker_bin)
			.args(["image", "ls"])
			.args(["--format", models::IMAGE_FMT])
			.output()
			.context(DOCKER_EXEC_ERROR)?;

		check_exit!(child);

		let mut images: Vec<String> = Vec::new();

		for line in child.stdout.lines() {
			let line = match line {
				Ok(v) => v,
				Err(_) => continue,
			};

			let image: models::Image = serde_json::from_str(&line)
				.context("Failed to parse docker output")?;

			images.push(image.name);
		}

		Ok(images)
	}

	fn make_image(&mut self, image_name: &str) -> Result<()> {
		let build_context = String::from("./");

		let status = process::Command::new(&self.docker_bin)
			.args(["build"])
			.args(["--pull", "--rm"])
			.args(["-t", image_name])
			.arg("--")
			.arg(build_context)
			.status()
			.context(DOCKER_EXEC_ERROR)?;

		if !status.success() {
			return Err(Error::new_with_context(
				format!(
					"error {}",
					status.code().unwrap_or_default()
				),
				"Error during image build",
			));
		}

		Ok(())
	}

	fn delete_image(&mut self, image_name: &str) -> Result<()> {
		let child = process::Command::new(&self.docker_bin)
			.args(["image", "rm"])
			.arg("--")
			.arg(image_name)
			.output()
			.context(DOCKER_EXEC_ERROR)?;

		check_exit!(child);

		Ok(())
	}

	fn list_machines(&self) -> Result<Vec<String>> {
		let child = process::Command::new(&self.docker_bin)
			.args(["container", "ls"])
			.args(["-a"])
			.args(["--format", models::CONTAINER_FMT])
			.output()
			.context(DOCKER_EXEC_ERROR)?;

		check_exit!(child);

		let mut containers: Vec<String> = Vec::new();

		for line in child.stdout.lines() {
			let line = match line {
				Ok(v) => v,
				Err(_) => continue,
			};

			let container: models::Container =
				serde_json::from_str(&line)
					.context("Failed to parse docker output")?;

			containers.push(container.name);
		}

		Ok(containers)
	}

	fn create_machine(
		&mut self,
		machine_name: &str,
		image_name: &str,
		opts: &GlobalOptions,
	) -> Result<()> {
		let child = process::Command::new(&self.docker_bin)
			.args(["container", "create"])
			.args(["-i", "-t"])
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
			.args([
				"--cap-drop=all",
				"--cap-add=CAP_SETUID",
				"--cap-add=CAP_SETGID",
				"--cap-add=CAP_SYS_CHROOT",
			])
			.arg("--")
			.arg(image_name)
			.output()
			.context(DOCKER_EXEC_ERROR)?;

		check_exit!(child);

		Ok(())
	}

	fn start_machine(&mut self, machine_name: &str) -> Result<()> {
		let status = process::Command::new(&self.docker_bin)
			.args(["container", "start"])
			.args(["-a", "-i"])
			.arg("--")
			.arg(machine_name)
			.status()
			.context(DOCKER_EXEC_ERROR)?;

		if !status.success() {
			return Err(Error::new_with_context(
				format!(
					"error {}",
					status.code().unwrap_or_default()
				),
				"Failed to attach to container",
			));
		}

		Ok(())
	}

	fn stop_machine(&mut self, machine_name: &str) -> Result<()> {
		let child = process::Command::new(&self.docker_bin)
			.args(["container", "kill"])
			.arg("--")
			.arg(machine_name)
			.output()
			.context(DOCKER_EXEC_ERROR)?;

		check_exit!(child);

		Ok(())
	}

	fn delete_machine(&mut self, machine_name: &str) -> Result<()> {
		let child = process::Command::new(&self.docker_bin)
			.args(["container", "rm"])
			.args(["-f", "-v"])
			.arg("--")
			.arg(machine_name)
			.output()
			.context(DOCKER_EXEC_ERROR)?;

		check_exit!(child);

		Ok(())
	}
}

const DOCKER_EXEC_ERROR: &str = "Failed to execute docker";

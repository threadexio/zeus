//! This is the zeus runtime driver for Docker.
//! Available user configuration is done by the
//! following environment variables:
//! - `DOCKER_BIN` - This must point to the docker cli tool. (default: `/usr/bin/docker`)

use zeus::{machine::*, *};

use std::env;
use std::io::BufRead;
use std::process;

mod models;

macro_rules! handle {
	($x:expr) => {
		match $x {
			Ok(v) => v,
			Err(e) => {
				return Err(format!(
					"{}: {}",
					"could not execute docker", e
				))
			},
		}
	};
	($x:expr, $msg:tt) => {
		match $x {
			Ok(v) => v,
			Err(e) => return Err(format!("{}: {}", $msg, e)),
		}
	};
}

macro_rules! check_exit {
	($output:expr) => {
		if !$output.status.success() {
			return Err(format!(
				"{}",
				String::from_utf8_lossy(&$output.stderr[..])
			));
		}
	};
}

#[derive(Default)]
pub struct DockerRuntime {
	docker_bin: String,
}

declare_runtime!(DockerRuntime, DockerRuntime::default);

impl IRuntime for DockerRuntime {
	fn name(&self) -> &'static str {
		"docker"
	}

	fn version(&self) -> &'static str {
		env!("CARGO_PKG_VERSION", "must be built with cargo")
	}

	fn rt_api_version(&self) -> u32 {
		1
	}

	fn init(&mut self) -> Result<()> {
		self.docker_bin = env::var("DOCKER_BIN")
			.unwrap_or(String::from("/usr/bin/docker"));

		Ok(())
	}

	fn exit(&mut self) {}

	fn list_images(&self) -> Result<Vec<String>> {
		let child = handle!(process::Command::new(&self.docker_bin)
			.args(["image", "ls"])
			.args(["--format", models::IMAGE_FMT])
			.output());

		check_exit!(child);

		let mut images: Vec<String> = Vec::new();

		for line in child.stdout.lines() {
			let line = match line {
				Ok(v) => v,
				Err(_) => continue,
			};

			let image: models::Image = handle!(
				serde_json::from_str(&line),
				"could not decode docker output"
			);

			images.push(image.name);
		}

		Ok(images)
	}

	fn make_image(&mut self, image_name: &str) -> Result<()> {
		let build_context = String::from("./");

		let status = handle!(process::Command::new(&self.docker_bin)
			.args(["build"])
			.args(["--pull", "--rm"])
			.args(["-t", image_name])
			.arg("--")
			.arg(build_context)
			.status());

		if !status.success() {
			return Err(format!("error during image build"));
		}

		Ok(())
	}

	fn delete_image(&mut self, image_name: &str) -> Result<()> {
		let child = handle!(process::Command::new(&self.docker_bin)
			.args(["image", "rm"])
			.arg("--")
			.arg(image_name)
			.output());

		check_exit!(child);

		Ok(())
	}

	fn list_machines(&self) -> Result<Vec<String>> {
		let child = handle!(process::Command::new(&self.docker_bin)
			.args(["container", "ls"])
			.args(["-a"])
			.args(["--format", models::CONTAINER_FMT])
			.output());

		check_exit!(child);

		let mut containers: Vec<String> = Vec::new();

		for line in child.stdout.lines() {
			let line = match line {
				Ok(v) => v,
				Err(_) => continue,
			};

			let container: models::Container = handle!(
				serde_json::from_str(&line),
				"could not decode docker output"
			);

			containers.push(container.name);
		}

		Ok(containers)
	}

	fn create_machine(
		&mut self,
		machine_name: &str,
		image_name: &str,
		config: &AppConfig,
	) -> Result<()> {
		let child = handle!(process::Command::new(&self.docker_bin)
			.args(["container", "create"])
			.args(["-i" /* "-t" */,])	// ISSUE: Allocating a pseudo terminal with -t gives pretty colors to
			                        	// the container output but sets some terminal settings with stty and
										// screws up output and user input.
			.args(["--name", machine_name])
			.args([
				"-v",
				"/var/cache/pacman/pkg:/var/cache/pacman/pkg:rw"
			])
			.args(["-v", &format!("{}:/build:rw", config.build_dir)])
			.args([
				"--cap-drop=all",
				"--cap-add=CAP_SETUID",
				"--cap-add=CAP_SETGID",
				"--cap-add=CAP_SYS_CHROOT",
			])
			.arg("--")
			.arg(image_name)
			.output());

		check_exit!(child);

		Ok(())
	}

	fn start_machine(&mut self, machine_name: &str) -> Result<()> {
		let child = handle!(process::Command::new(&self.docker_bin)
			.args(["container", "start"])
			.arg("--")
			.arg(machine_name)
			.output());

		check_exit!(child);

		let status = handle!(process::Command::new(&self.docker_bin)
			.args(["container", "attach"])
			.arg("--")
			.arg(machine_name)
			.status());

		if !status.success() {
			return Err(format!("failed to attach to machine"));
		}

		Ok(())
	}

	fn stop_machine(&mut self, machine_name: &str) -> Result<()> {
		let child = handle!(process::Command::new(&self.docker_bin)
			.args(["container", "kill"])
			.arg("--")
			.arg(machine_name)
			.output());

		check_exit!(child);

		Ok(())
	}

	fn delete_machine(&mut self, machine_name: &str) -> Result<()> {
		let child = handle!(process::Command::new(&self.docker_bin)
			.args(["container", "rm"])
			.args(["-f", "-v"])
			.arg("--")
			.arg(machine_name)
			.output());

		check_exit!(child);

		Ok(())
	}
}

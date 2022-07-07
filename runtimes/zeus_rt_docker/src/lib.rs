//! This is the zeus runtime driver for Docker.
//! Available user configuration is done by the
//! following environment variables:
//! - `DOCKER_BIN` - This must point to the docker cli tool. (default: `/usr/bin/docker`)

use zeus::{machine::*, *};

use std::env;
use std::io::BufRead;
use std::process;

mod command;
mod models;

macro_rules! handle {
	($x:expr, $msg:tt) => {
		match $x {
			Ok(v) => v,
			Err(e) => return Err(format!("{}: {}", $msg, e)),
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

	fn list_images(&self) -> Result<Vec<BoxedImage>> {
		let process::Output { status, stdout, stderr, .. } = handle!(
			process::Command::new(&self.docker_bin)
				.arg("image")
				.arg("ls")
				.arg("-a")
				.arg("--format")
				.arg(models::IMAGE_FMT)
				.output(),
			"could not execute docker"
		);

		if !command::check_exit_ok(status) {
			return Err(format!(
				"docker exited with {}: {}",
				status.code().unwrap_or_default(),
				String::from_utf8_lossy(&stderr)
			));
		}

		let mut images: Vec<BoxedImage> = Vec::new();

		for line in stdout.lines() {
			let line = handle!(line, "could not decode line");

			let image: models::Image = handle!(
				serde_json::from_str(&line),
				"could not decode docker output"
			);

			images.push(Box::new(image));
		}

		Ok(images)
	}

	fn create_image(
		&mut self,
		image_name: &str,
	) -> Result<BoxedImage> {
		for image in self.list_images()? {
			if image_name == image.name() {
				return Err(format!("image already exists"));
			}
		}

		let build_context = String::from("./");

		let status = handle!(
			process::Command::new(&self.docker_bin)
				.arg("build")
				.arg("--pull")
				.arg("--rm")
				.arg("-t")
				.arg(image_name)
				.arg("--")
				.arg(build_context)
				.status(),
			"could not execute docker"
		);

		if !command::check_exit_ok(status) {
			return Err(format!(
				"docker exited with: {}",
				status.code().unwrap_or_default()
			));
		}

		for image in self.list_images()? {
			if image_name == image.name() {
				return Ok(Box::new(models::Image {
					id: image.id().to_string(),
					name: image_name.to_string(),
				}));
			}
		}

		return Err(format!("unknown error during build"));
	}

	fn update_image(&mut self, image: &Image) -> Result<()> {
		let build_context = String::from("./");

		let status = handle!(
			process::Command::new(&self.docker_bin)
				.arg("build")
				.arg("--pull")
				.arg("--rm")
				.arg("-t")
				.arg(image.name())
				.arg("--")
				.arg(build_context)
				.status(),
			"could not execute docker"
		);

		if !command::check_exit_ok(status) {
			return Err(format!(
				"docker exited with: {}",
				status.code().unwrap_or_default()
			));
		}

		Ok(())
	}

	fn delete_image(&mut self, image: BoxedImage) -> Result<()> {
		let process::Output { status, .. } = handle!(
			process::Command::new(&self.docker_bin)
				.arg("image")
				.arg("rm")
				.arg("--")
				.arg(image.id())
				.output(),
			"could not execute docker"
		);

		if !command::check_exit_ok(status) {
			return Err(format!(
				"docker exited with: {}",
				status.code().unwrap_or_default()
			));
		}

		Ok(())
	}

	fn list_machines(&self) -> Result<Vec<BoxedMachine>> {
		let process::Output { status, stdout, stderr, .. } = handle!(
			process::Command::new(&self.docker_bin)
				.arg("container")
				.arg("ls")
				.arg("-a")
				.arg("--format")
				.arg(models::CONTAINER_FMT)
				.output(),
			"could not execute docker"
		);

		if !command::check_exit_ok(status) {
			return Err(format!(
				"docker exited with {}: {}",
				status.code().unwrap_or_default(),
				String::from_utf8_lossy(&stderr)
			));
		}

		let mut containers: Vec<BoxedMachine> = Vec::new();

		for line in stdout.lines() {
			let line = handle!(line, "could not decode line");

			let container: models::Container = handle!(
				serde_json::from_str(&line),
				"could not decode docker output"
			);

			containers.push(Box::new(container));
		}

		Ok(containers)
	}

	fn create_machine(
		&mut self,
		machine_name: &str,
		image: &Image,
		config: &AppConfig,
	) -> Result<BoxedMachine> {
		let process::Output { status, stdout, .. } = handle!(
			process::Command::new(&self.docker_bin)
				.arg("container")
				.arg("create")
				//.arg("-t")
				.arg("-i")
				.arg("--name")
				.arg(machine_name)
				.arg("-v")
				.arg("/var/cache/pacman/pkg:/var/cache/pacman/pkg:rw")
				.arg("-v")
				.arg(format!("{}:/build:rw", config.build_dir))
				.arg("--cap-drop=all")
				.arg("--cap-add=CAP_SETUID")
				.arg("--cap-add=CAP_SETGID")
				.arg("--")
				.arg(image.id())
				.output(),
			"could not execute docker"
		);

		if !command::check_exit_ok(status) {
			return Err(format!(
				"docker exited with: {}",
				status.code().unwrap_or_default(),
			));
		}

		let container_id = &String::from_utf8_lossy(&stdout)[..10];

		Ok(Box::new(models::Container {
			id: container_id.to_string(),
			name: machine_name.to_string(),
			image: image.id().to_string(),
		}))
	}

	fn start_machine(&mut self, machine: &Machine) -> Result<()> {
		let process::Output { status, .. } = handle!(
			process::Command::new(&self.docker_bin)
				.arg("container")
				.arg("start")
				.arg("--")
				.arg(machine.id())
				.output(),
			"could not execute docker"
		);

		if !command::check_exit_ok(status) {
			return Err(format!(
				"docker exited with: {}",
				status.code().unwrap_or_default()
			));
		}

		Ok(())
	}

	fn stop_machine(&mut self, machine: &Machine) -> Result<()> {
		let process::Output { status, .. } = handle!(
			process::Command::new(&self.docker_bin)
				.arg("container")
				.arg("kill")
				.arg("--")
				.arg(machine.id())
				.output(),
			"could not execute docker"
		);

		if !command::check_exit_ok(status) {
			return Err(format!(
				"docker exited with: {}",
				status.code().unwrap_or_default()
			));
		}

		Ok(())
	}

	fn attach_machine(&mut self, machine: &Machine) -> Result<()> {
		// todo: attach container to a pty for pretty colors

		handle!(
			process::Command::new(&self.docker_bin)
				.arg("container")
				.arg("attach")
				.arg("--")
				.arg(machine.id())
				.spawn(),
			"could not execute docker"
		);

		Ok(())
	}

	fn execute_command(
		&mut self,
		machine: &Machine,
		command: &str,
	) -> Result<i32> {
		let process::Output { status, .. } = handle!(
			process::Command::new(&self.docker_bin)
				.arg("container")
				.arg("exec")
				.arg("-i")
				//.arg("-t")
				.arg("--")
				.arg(machine.id())
				.arg(command)
				.output(),
			"could not execute docker"
		);

		Ok(status.code().unwrap_or_default())
	}

	fn delete_machine(
		&mut self,
		machine: BoxedMachine,
	) -> Result<()> {
		let process::Output { status, .. } = handle!(
			process::Command::new(&self.docker_bin)
				.arg("container")
				.arg("rm")
				.arg("-f")
				.arg("-v")
				.arg("--")
				.arg(machine.id())
				.output(),
			"could not execute docker"
		);

		if !command::check_exit_ok(status) {
			return Err(format!(
				"docker exited with: {}",
				status.code().unwrap_or_default()
			));
		}

		Ok(())
	}
}

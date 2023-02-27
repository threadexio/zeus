//! This is the zeus runtime driver for Docker.
//! Available user configuration is done by the
//! following environment variables:
//! - `DOCKER_BIN` - This must point to the docker cli tool. (default: `/usr/bin/docker`)
use zeus::runtime::*;

use std::env;
use std::path::{Path, PathBuf};
use std::process::{self, Stdio};

fn run_docker(
	term: &mut Terminal,
	mut cmd: process::Command,
) -> Result<process::Output> {
	let cmdline = {
		let mut c = String::with_capacity(1024);

		c.push_str(&cmd.get_program().to_string_lossy());
		c.push(' ');
		for arg in cmd.get_args() {
			c.push_str(&arg.to_string_lossy());
			c.push(' ');
		}
		c.pop();
		c
	};

	term.debug(format!("Running: {cmdline}"));

	let output = match cmd.output() {
		Ok(v) => v,
		Err(e) => {
			return Err(Error::new(e.to_string()));
		},
	};

	if !output.status.success() {
		return Err(Error::new(format!(
			"docker exited with: {}",
			output.status.code().unwrap_or(-1)
		)));
	}

	Ok(output)
}

pub struct DockerRuntime {
	docker_bin: PathBuf,
	build_context: PathBuf,
	dockerfile: PathBuf,
	pacman_cache: PathBuf,
}

impl Default for DockerRuntime {
	fn default() -> Self {
		Self {
			docker_bin: Path::new("/usr/bin/docker").to_path_buf(),
			build_context: Path::new("./").to_path_buf(),
			dockerfile: Path::new("Dockerfile").to_path_buf(),
			pacman_cache: Path::new("/var/cache/pacman/pkg")
				.to_path_buf(),
		}
	}
}

impl IRuntime for DockerRuntime {
	fn name(&self) -> &'static str {
		"docker"
	}

	fn version(&self) -> &'static str {
		env!("CARGO_PKG_VERSION", "must be built with cargo")
	}

	fn init(
		&mut self,
		config: &GlobalConfig,
		term: &mut Terminal,
	) -> Result<()> {
		let opts = &config.runtime_opts;

		if let Some(v) = env::var_os("DOCKER_BIN")
			.map(|x| Path::new(&x).to_path_buf())
			.or(opts
				.get("Program")
				.map(|x| Path::new(&x).to_path_buf()))
		{
			self.docker_bin = v;
		}

		if let Some(v) = opts.get("Context") {
			self.build_context = v.into();
		}

		if let Some(v) = opts.get("Dockerfile") {
			self.dockerfile = v.into();
		}

		if let Some(v) = opts.get("PacmanCache") {
			self.pacman_cache = v.into();
		}

		// this should fail if we dont have permission to access docker
		let mut cmd = process::Command::new(&self.docker_bin);
		cmd.stdin(Stdio::null())
			.stdout(Stdio::null())
			.stderr(Stdio::inherit())
			.arg("version");
		run_docker(term, cmd)?;

		Ok(())
	}

	fn exit(&mut self) {}

	fn create_image(
		&mut self,
		config: &GlobalConfig,
		term: &mut Terminal,
	) -> Result<()> {
		let mut cmd = process::Command::new(&self.docker_bin);
		cmd.stdin(Stdio::inherit())
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit())
			.arg("build")
			.arg("--pull")
			.arg("--force-rm")
			.arg("-t")
			.arg(&config.machine_image)
			.arg("-f")
			.arg(&self.dockerfile)
			.arg("--")
			.arg(&self.build_context);
		run_docker(term, cmd)?;

		Ok(())
	}

	fn create_machine(
		&mut self,
		config: &GlobalConfig,
		term: &mut Terminal,
	) -> Result<()> {
		let mut cmd = process::Command::new(&self.docker_bin);
		cmd.stdin(Stdio::null())
			.stdout(Stdio::null())
			.stderr(Stdio::null())
			.arg("container")
			.arg("rm")
			.arg("--")
			.arg(&config.machine_name);
		let _ = run_docker(term, cmd);

		let mut cmd = process::Command::new(&self.docker_bin);
		cmd.stdin(Stdio::inherit())
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit())
			.arg("container")
			.arg("create")
			.arg("-i")
			.arg("-t")
			.arg("--name")
			.arg(&config.machine_name)
			.arg("-v")
			.arg(format!(
				"{}:/var/cache/pacman/pkg:rw",
				self.pacman_cache.to_string_lossy()
			))
			.arg("-v")
			.arg(format!(
				"{}:/build:rw",
				config.build_dir.to_string_lossy()
			))
			.arg("--cap-drop=all")
			.arg("--cap-add=CAP_SETUID")
			.arg("--cap-add=CAP_SETGID")
			.arg("--cap-add=CAP_SYS_CHROOT")
			.arg("--")
			.arg(&config.machine_image);
		run_docker(term, cmd)?;

		Ok(())
	}

	fn start_machine(
		&mut self,
		config: &GlobalConfig,
		term: &mut Terminal,
	) -> Result<()> {
		let mut cmd = process::Command::new(&self.docker_bin);
		cmd.stdin(Stdio::inherit())
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit())
			.arg("container")
			.arg("start")
			.arg("-a")
			.arg("-i")
			.arg("--")
			.arg(&config.machine_name);
		run_docker(term, cmd)?;

		Ok(())
	}
}

runtime!(DockerRuntime::default);

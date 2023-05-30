use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn set_var<K, V>(k: &K, v: &V)
where
	K: ?Sized + Display,
	V: ?Sized + Display,
{
	println!("cargo:rustc-env={k}={v}");
}

type Metadata = serde_json::Value;

fn cargo_metadata() -> Metadata {
	let output = Command::new("cargo")
		.arg("metadata")
		.arg("--format-version=1")
		.stdin(Stdio::null())
		.stderr(Stdio::inherit())
		.output()
		.unwrap();
	if !output.status.success() {
		panic!("cargo exited with: {:?}", output.status.code())
	}

	serde_json::from_slice(&output.stdout).unwrap()
}

fn version() -> String {
	let output = Command::new("git")
		.arg("describe")
		.arg("--tags")
		.arg("--always")
		.arg("--dirty")
		.arg("--broken")
		.stdin(Stdio::null())
		.stderr(Stdio::inherit())
		.output()
		.unwrap();
	if !output.status.success() {
		panic!(
			"git exited with: {}",
			output.status.code().unwrap_or(-1)
		)
	}

	let output = String::from_utf8(output.stdout)
		.expect("git emitted invalid utf8");
	let output = output.trim().replace('-', "_");
	output
}

fn rustc_version() -> String {
	let output = Command::new("rustc")
		.arg("--version")
		.stdin(Stdio::null())
		.stderr(Stdio::inherit())
		.output()
		.unwrap();
	if !output.status.success() {
		panic!("rustc exited with: {:?}", output.status.code())
	}
	let output = String::from_utf8(output.stdout).unwrap();
	let output = output.trim().to_string();
	output
}

fn profile() -> String {
	match env::var("PROFILE").unwrap().as_str() {
		"debug" => "dev".to_string(),
		p => p.to_string(),
	}
}

fn load_profile(metadata: &Metadata) {
	let profile = profile();
	let workspace_root = metadata["workspace_root"].as_str().unwrap();

	set_var("PROFILE", &profile);

	let mut file = PathBuf::new();
	file.push(workspace_root);
	file.push("profiles");
	file.push(format!("{}.json", &profile));
	println!("cargo:rerun-if-changed={}", file.display());

	let profile_vars: HashMap<String, String> =
		serde_json::from_reader(
			File::options().read(true).open(file).unwrap(),
		)
		.unwrap();

	for (k, v) in &profile_vars {
		set_var(k, v);
	}
}

fn load_build_info() {
	let version = version();
	set_var("VERSION", &version);

	let build_info = &format!(
		"{}@{} ({}) {}",
		whoami::username_os().to_string_lossy(),
		whoami::hostname_os().to_string_lossy(),
		rustc_version(),
		chrono::Utc::now().format("%a %b %d %I:%M:%S %p %Z %Y")
	);
	set_var("BUILD_INFO", &build_info);

	set_var("RUSTC_VERSION", &rustc_version());
}

fn main() {
	let metadata = cargo_metadata();

	load_profile(&metadata);
	load_build_info();
}

use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::path::Path;
use std::process::{Command, Stdio};

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

fn profile() -> String {
	match env::var("PROFILE").unwrap().as_str() {
		"debug" => "dev".to_string(),
		p => p.to_string(),
	}
}

fn build_info() -> String {
	format!(
		"{}@{} ({}) {}",
		whoami::username_os().to_string_lossy(),
		whoami::hostname_os().to_string_lossy(),
		rustc_version(),
		chrono::Utc::now().format("%a %b %d %I:%M:%S %p %Z %Y")
	)
}

fn set_var<K, V>(k: &K, v: &V)
where
	K: ?Sized + Display,
	V: ?Sized + Display,
{
	println!("cargo:rustc-env={k}={v}");
}

fn main() {
	set_var("PROFILE", &profile());
	set_var("RUSTC_VERSION", &rustc_version());
	set_var("VERSION", &version());
	set_var("BUILD_INFO", &build_info());

	let file =
		Path::new("profiles").join(format!("{}.json", profile()));
	println!("cargo:rerun-if-changed={}", file.display());

	let profile_vars: HashMap<String, String> =
		serde_json::from_reader(
			File::options().read(true).open(file).unwrap(),
		)
		.unwrap();

	profile_vars
		.iter()
		.for_each(|(var, value)| set_var(var, value));
}

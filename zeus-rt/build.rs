use std::fmt::Display;
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

fn set_var<K, V>(k: &K, v: &V)
where
	K: ?Sized + Display,
	V: ?Sized + Display,
{
	println!("cargo:rustc-env={k}={v}");
}

fn main() {
	set_var("RUSTC_VERSION", &rustc_version());
}

use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

fn set_var<K, V>(k: &K, v: &V)
where
	K: ?Sized + Display,
	V: ?Sized + Display,
{
	println!("cargo:rustc-env={k}={v}");
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

fn main() {
	let f = Path::new(&env::var("OUT_DIR").unwrap()).join("abi.rs");
	set_var("GENERATED_OUT", &f.to_string_lossy());

	let mut f = File::options()
		.write(true)
		.truncate(true)
		.create(true)
		.open(f)
		.unwrap();

	let mut input: Vec<u8> = Vec::with_capacity(1024);
	input.extend(version().bytes());
	input.extend(rustc_version().bytes());

	let hash = xxhash_rust::xxh3::xxh3_64(&input);
	writeln!(
		&mut f,
		r"#[allow(dead_code)] pub const VERSION: u64 = {};",
		hash
	)
	.unwrap();
}

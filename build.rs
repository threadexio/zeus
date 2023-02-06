use std::env;
use std::fmt::Display;
use std::path::Path;
use std::process::Command;

fn set_var<K, V>(k: &K, v: &V)
where
	K: ?Sized + Display,
	V: ?Sized + Display,
{
	println!("cargo:rustc-env={k}={v}");
	println!("cargo:rerun-if-env-changed={k}");
}

fn run_script(script: &str) -> String {
	let c = Command::new(script)
		.output()
		.unwrap_or_else(|_| panic!("cannot run {script}"));

	if !c.status.success() {
		panic!(
			"
{script} exited with: {}
Stdout:
{}
Stderr:
{}",
			c.status.code().unwrap_or(-42),
			String::from_utf8_lossy(&c.stdout),
			String::from_utf8_lossy(&c.stderr),
		);
	}

	String::from_utf8(c.stdout).unwrap_or_else(|_| {
		panic!("{script}: cannot contain invalid utf8")
	})
}

fn main() {
	let build_root = Path::new(&env::var("OUT_DIR").unwrap())
		.join("../../..") // a very hacky way to get the root build directory (`target/debug`)
		.canonicalize()
		.expect("build_dir does not exist");

	let build_root = pathdiff::diff_paths(
		&build_root,
		env::current_dir().expect("cannot get cwd"),
	)
	.unwrap_or(build_root);

	let profile =
		match build_root.file_name().unwrap().to_str().unwrap() {
			"debug" => "dev",
			s => s,
		}
		.to_string();

	set_var("PROFILE", &profile);
	set_var("VERSION", &run_script("scripts/version.sh"));
	set_var("BUILD_INFO", &run_script("scripts/build_info.sh"));

	let file = Path::new("profiles").join(format!("{profile}.env"));

	println!("cargo:rerun-if-changed={}", file.display());
	let raw_env =
		std::fs::read_to_string(file).expect("cannot read profile");

	for line in raw_env.lines() {
		let line = line.trim();

		if let Some((k, v)) = line.split_once('=') {
			set_var(k, v);
		}
	}

	let _ = std::fs::remove_file("build");
	std::os::unix::fs::symlink(build_root, "./build")
		.expect("unable to link latest build to ./build");
}

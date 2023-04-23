use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;

lazy_static::lazy_static! {
	static ref RUSTC_VERSION: String = {
		let output =
		Command::new("rustc").arg("--version").output().unwrap();
		if !output.status.success() { panic!("rustc exited with: {:?}", output.status.code()) }
		String::from_utf8_lossy(&output.stdout).trim().to_string()
	};

	static ref BUILD_ROOT_ABS: PathBuf =
		Path::new(&env::var("OUT_DIR").unwrap())
			.join("../../..")
			.canonicalize()
			.expect("OUT_DIR does not exist");

	static ref BUILD_ROOT_REL: PathBuf = pathdiff::diff_paths(
		&*BUILD_ROOT_ABS,
		env::current_dir().expect("cannot get cwd"),
	)
	.expect("cannot diff build root and cwd");

	static ref PROFILE: String = {
		let build_dir_name = (*BUILD_ROOT_ABS).file_name().expect("cannot get build root file name").to_str().expect("build root file name is not valid utf8");

		match build_dir_name {
			"debug" => "dev".to_string(),
			_ => build_dir_name.to_string()
		}
	};


}

fn version() -> String {
	let output = Command::new("git")
		.arg("describe")
		.arg("--tags")
		.arg("--always")
		.arg("--dirty")
		.arg("--broken")
		.output()
		.unwrap();
	if !output.status.success() {
		panic!("git exited with: {:?}", output.status.code())
	}

	let output = String::from_utf8(output.stdout)
		.expect("git emitted invalid utf8");

	output.trim().replace('-', "_")
}

fn build_info() -> String {
	format!(
		"{}@{} ({}) {}",
		whoami::username_os().to_string_lossy(),
		whoami::hostname_os().to_string_lossy(),
		*RUSTC_VERSION,
		chrono::Utc::now().format("%a %b %d %I:%M:%S %p %Z %Y")
	)
}

fn setup_build_env() {
	// symlink build to ./build
	{
		let build_link = Path::new("build");
		let _ = std::fs::remove_file(build_link);
		std::os::unix::fs::symlink(&*BUILD_ROOT_REL, build_link)
			.unwrap_or_else(|_| {
				panic!(
					"unable to link latest build to {:?}",
					build_link
				)
			});
	}

	// load env vars from profile
	{
		let file =
			Path::new("profiles").join(format!("{}.json", *PROFILE));
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
}

fn set_var<K, V>(k: &K, v: &V)
where
	K: ?Sized + Display,
	V: ?Sized + Display,
{
	println!("cargo:rustc-env={k}={v}");
}

fn main() {
	set_var("PROFILE", &*PROFILE);
	set_var("RUSTC_VERSION", &*RUSTC_VERSION);
	set_var("VERSION", &version());
	set_var("BUILD_INFO", &build_info());

	setup_build_env()
}

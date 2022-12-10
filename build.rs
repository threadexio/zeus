use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::fs::File;
use std::path::Path;
use std::process::Command;

fn set_var<K, V>(k: &K, v: &V)
where
	K: ?Sized + Display,
	V: ?Sized + Display,
{
	println!("cargo:rustc-env={}={}", k, v);
}

fn version() -> String {
	let c = Command::new("scripts/version.sh")
		.output()
		.expect("cannot run scripts/version.sh");

	String::from_utf8(c.stdout)
		.expect("the version cannot contain invalid utf8")
}

fn build_info() -> String {
	let c = Command::new("scripts/build_info.sh")
		.output()
		.expect("cannot run scripts/build_info.sh");

	String::from_utf8(c.stdout)
		.expect("the build info cannot contain invalid utf8")
}

fn profile() -> String {
	env::var("PROFILE").expect("cargo did not set PROFILE")
}

fn main() {
	let profile = profile();
	set_var("PROFILE", &profile);
	set_var("VERSION", &version());
	set_var("BUILD_INFO", &build_info());

	let file = Path::new("profiles").join(format!(
		"{}.json",
		match profile.as_str() {
			"debug" => "dev",
			p => p,
		}
	));

	println!("{:?}", &file);
	let file = File::open(file).expect("cannot open profile config");
	let fields: HashMap<String, String> =
		serde_json::from_reader(file).unwrap();

	for (k, v) in fields.iter() {
		set_var(k, v);
	}

	// symlink the latest build to ./build
	// so we can install directly from the
	// overlay with symlinks

	let build_root = Path::new(&env::var("OUT_DIR").unwrap())
	.join("../../..") // a very hacky way to get the root build directory (`target/debug`)
	.canonicalize()
	.expect("build_dir does not exist");

	let build_path = pathdiff::diff_paths(
		&build_root,
		env::current_dir().expect("cannot get cwd"),
	)
	.unwrap_or(build_root);

	let _ = std::fs::remove_file("./build");
	std::os::unix::fs::symlink(build_path, "./build")
		.expect("unable to link latest build to /build")
}

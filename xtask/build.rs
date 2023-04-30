use std::env;
use std::path::Path;

fn main() {
	let out_dir = env::var("OUT_DIR").unwrap();
	let target_dir = Path::new(&out_dir).ancestors().nth(4).unwrap();

	println!(
		"cargo:rustc-env=TARGET_DIR={}",
		target_dir.to_str().unwrap()
	);
}

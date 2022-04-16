use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[allow(dead_code)]
pub const PROGRAM_NAME: &'static str = "zeus";
pub const PROGRAM_DESC: &'static str = "Containerized AUR helper";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub verbose: bool,
    pub force: bool,
    pub upgrade: bool,

    // Package builder image context
    pub builder_archive: String,
    // Package builder image dockerfile
    pub builder_dockerfile: String,
    // Package builder image name (name:[tag])
    pub builder_image: String,

    // Packages to perform operations on
    pub packages: Vec<String>,
    // Package build directory in host
    pub build_dir: String,
    // makepkg build args
    pub build_args: Vec<String>,
}

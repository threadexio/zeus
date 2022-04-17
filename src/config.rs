use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[allow(dead_code)]
pub const PROGRAM_NAME: &'static str = "zeus";

#[allow(dead_code)]
pub const PROGRAM_DESC: &'static str = "Containerized AUR helper";

#[cfg(debug_assertions)]
pub const PROGRAM_VERSION: &'static str = concat!(env!("VERSION"), "-", "dbg");

#[cfg(not(debug_assertions))]
pub const PROGRAM_VERSION: &'static str = concat!(env!("VERSION"), "-", "rls");

#[derive(Debug, Serialize, Deserialize)]
pub struct Builder {
    // Package builder image context
    pub archive: String,
    // Package builder image dockerfile
    pub dockerfile: String,
    // Package builder image name (name:[tag])
    pub image: String,
    // Builder container name
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // Packages to perform operations on
    pub packages: Vec<String>,

    pub force: bool,
    pub upgrade: bool,

    pub builder: Builder,

    // Package build directory in host
    pub build_dir: String,
    // makepkg build args
    pub build_args: Vec<String>,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            archive: option_env!("DEFAULT_ARCHIVE")
                .unwrap_or("/usr/share/zeus/builder.tar.gz")
                .to_owned(),
            dockerfile: option_env!("DEFAULT_DOCKERFILE")
                .unwrap_or("Dockerfile")
                .to_owned(),
            image: option_env!("DEFAULT_IMAGE")
                .unwrap_or("zeus-builder:latest")
                .to_owned(),
            name: option_env!("DEFAULT_NAME")
                .unwrap_or("zeus-builder")
                .to_owned(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            packages: vec![],

            force: false,
            upgrade: false,

            builder: Builder::default(),

            build_dir: option_env!("DEFAULT_BUILDDIR")
                .unwrap_or("/var/cache/aur")
                .to_owned(),

            build_args: vec![],
        }
    }
}

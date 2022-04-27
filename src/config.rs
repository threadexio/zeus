use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use const_format::formatcp;
use default_env::default_env;

#[allow(dead_code)]
pub const PROGRAM_NAME: &'static str = "zeus";

#[allow(dead_code)]
pub const PROGRAM_DESC: &'static str = env!("CARGO_PKG_DESCRIPTION");

#[cfg(debug_assertions)]
const BUILD_TYPE: &'static str = "dbg";

#[cfg(not(debug_assertions))]
const BUILD_TYPE: &'static str = "rls";

pub const PROGRAM_VERSION: &'static str =
    formatcp!("{}-{BUILD_TYPE}", default_env!("VERSION", "unknown"));

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

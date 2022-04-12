use bollard::Docker;
use std::fmt;

pub const PROGRAM_NAME: &'static str = "zeus";
pub const PROGRAM_DESC: &'static str = "Containerized AUR helper";

pub struct Config {
    // Docker instance
    pub docker: Docker,

    pub verbose: bool,
    pub force: bool,

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
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("verbose", &self.verbose)
            .field("force", &self.force)
            .field("builder_archive", &self.builder_archive)
            .field("builder_dockerfile", &self.builder_dockerfile)
            .field("builder_image", &self.builder_image)
            .field("packages", &self.packages)
            .field("build_dir", &self.build_dir)
            .finish()
    }
}

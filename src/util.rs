use std::fs;
use std::io::Error;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};

use fs4::FileExt;

pub struct Lockfile {
    file: fs::File,
    blocking: bool,
}

impl Lockfile {
    pub fn new(path: &Path) -> Result<Self, Error> {
        Ok(Self {
            file: fs::File::create(path)?,
            blocking: true,
        })
    }

    pub fn set_blocking(&mut self, mode: bool) {
        self.blocking = mode;
    }

    pub fn lock(&self) -> Result<(), Error> {
        if self.blocking {
            self.file.lock_exclusive()
        } else {
            self.file.try_lock_exclusive()
        }
    }

    pub fn unlock(&self) -> Result<(), Error> {
        self.file.unlock()
    }
}

impl Drop for Lockfile {
    fn drop(&mut self) {
        let _ = self.unlock();
    }
}

pub struct LocalListener {
    pub listener: UnixListener,
    path: PathBuf,
}

impl LocalListener {
    pub fn new(path: &Path, mode: Option<u32>) -> Result<Self, Error> {
        let _ = fs::remove_file(path);

        Ok(Self {
            path: path.to_owned(),
            listener: match UnixListener::bind(path) {
                Ok(v) => {
                    if let Some(m) = mode {
                        fs::set_permissions(path, fs::Permissions::from_mode(m))?
                    }
                    v
                }
                Err(e) => {
                    return Err(e);
                }
            },
        })
    }
}

impl Drop for LocalListener {
    fn drop(&mut self) {
        let _ = fs::remove_file(self.path.as_path());
    }
}

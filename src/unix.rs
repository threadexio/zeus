use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::{UnixListener, UnixStream};

use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use channels::{Receiver, Sender};

pub struct LocalListener {
	listener: UnixListener,
	path: PathBuf,
}

#[allow(dead_code)]
impl LocalListener {
	pub fn new<P: AsRef<Path>>(
		path: P,
		mode: u32,
	) -> io::Result<Self> {
		let _ = fs::remove_file(&path);

		let listener = UnixListener::bind(&path)?;

		fs::set_permissions(&path, fs::Permissions::from_mode(mode))?;

		Ok(Self { path: path.as_ref().to_path_buf(), listener })
	}

	pub fn accept<T: Serialize + DeserializeOwned>(
		&self,
	) -> io::Result<(Sender<T, UnixStream>, Receiver<T, UnixStream>)>
	{
		let (stream, _) = self.listener.accept()?;

		Ok(channels::channel::<T, _>(stream))
	}
}

impl std::ops::Deref for LocalListener {
	type Target = UnixListener;

	fn deref(&self) -> &Self::Target {
		&self.listener
	}
}

impl std::ops::DerefMut for LocalListener {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.listener
	}
}

impl Drop for LocalListener {
	fn drop(&mut self) {
		let _ = fs::remove_file(self.path.as_path());
	}
}

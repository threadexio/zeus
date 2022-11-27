use std::fs;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};

use super::error::*;
use super::Message;

pub struct Listener {
	path: PathBuf,
	tx: channels::Sender<Message, UnixStream>,
	rx: channels::Receiver<Message, UnixStream>,
}

impl Listener {
	pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
		let _ = fs::remove_file(&path);

		let listener = UnixListener::bind(&path)?;

		let (connection, _) = listener.accept()?;

		let (tx, rx) = channels::channel(connection);

		Ok(Self { path: path.as_ref().to_path_buf(), tx, rx })
	}

	pub fn path(&self) -> &Path {
		&self.path
	}

	pub fn close(self) {}

	pub fn send(&mut self, m: Message) -> Result<()> {
		self.tx.send(m)?;

		Ok(())
	}

	pub fn recv(&mut self) -> Result<Message> {
		Ok(self.rx.recv()?)
	}
}

impl Drop for Listener {
	fn drop(&mut self) {
		let _ = fs::remove_file(self.path.as_path());
	}
}

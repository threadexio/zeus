use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};

use super::error::*;
use super::Message;

pub struct Client {
	path: PathBuf,
	tx: channels::Sender<Message, UnixStream>,
	rx: channels::Receiver<Message, UnixStream>,
}

impl Client {
	pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
		let connection = UnixStream::connect(&path)?;

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
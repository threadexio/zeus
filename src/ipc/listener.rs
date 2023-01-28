use std::fs;
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use super::Message;

pub struct Listener<'a> {
	path: PathBuf,
	tx: channels::Sender<'a, Message>,
	rx: channels::Receiver<'a, Message>,
}

impl Listener<'_> {
	pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
		let _ = fs::remove_file(&path);

		let listener = UnixListener::bind(&path)?;

		let (connection, _) = listener.accept()?;

		let (tx, rx) = channels::channel(
			connection
				.try_clone()
				.context("Unable to clone unix connection")?,
			connection,
		);

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

impl Drop for Listener<'_> {
	fn drop(&mut self) {
		let _ = fs::remove_file(self.path.as_path());
	}
}

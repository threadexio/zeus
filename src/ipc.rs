use std::fs;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};

use crate::aur::Package;
use crate::config::Config;
use crate::error::*;

type Sender = channels::Sender<Message, UnixStream>;
type Receiver = channels::Receiver<Message, UnixStream>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Message {
	Init(Config),
	Packages(Vec<Package>),
	End,
}

pub struct Listener {
	path: PathBuf,
	tx: Sender,
	rx: Receiver,
}

#[allow(dead_code)]
impl Listener {
	pub fn new(path: PathBuf) -> Result<Self> {
		let _ = fs::remove_file(&path);

		let listener = UnixListener::bind(&path).context(format!(
			"unable to bind on {}",
			path.display()
		))?;

		let (connection, _) = listener.accept().context(format!(
			"unable to accept incoming stream on {}",
			path.display()
		))?;

		let (tx, rx) = channels::channel::<Message, _>(connection);

		Ok(Self { path, tx, rx })
	}

	pub fn path(&self) -> &Path {
		&self.path
	}

	pub fn close(self) {
		drop(self)
	}

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
		let _ = self.tx.send(Message::End);
		let _ = fs::remove_file(self.path.as_path());
	}
}

pub struct Client {
	path: PathBuf,
	tx: Sender,
	rx: Receiver,
}

#[allow(dead_code)]
impl Client {
	pub fn new(path: PathBuf) -> Result<Self> {
		let connection = UnixStream::connect(&path)?;

		let (tx, rx) = channels::channel(connection);

		Ok(Self { path, tx, rx })
	}

	pub fn path(&self) -> &Path {
		&self.path
	}

	pub fn close(self) {
		drop(self)
	}

	pub fn send(&mut self, m: Message) -> Result<()> {
		self.tx.send(m)?;

		Ok(())
	}

	pub fn recv(&mut self) -> Result<Message> {
		Ok(self.rx.recv()?)
	}
}

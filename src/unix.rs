use std::fs;
use std::io;
use std::io::prelude::*;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::SocketAddr;
use std::os::unix::net::{UnixListener, UnixStream};

use bincode::Options;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

pub struct Channel<T, S>
where
	T: Serialize + DeserializeOwned,
	S: Read + Write,
{
	stream: S,
	_p: PhantomData<T>,
}

#[allow(dead_code)]
impl<T, S> Channel<T, S>
where
	T: Serialize + DeserializeOwned,
	S: Read + Write,
{
	pub fn new(stream: S) -> Self {
		Self { _p: PhantomData, stream }
	}

	pub fn stream<'a>(&'a mut self) -> &'a mut S {
		&mut self.stream
	}

	pub fn send(&mut self, obj: T) -> io::Result<()> {
		let data = bincode::DefaultOptions::new()
			.serialize(&obj)
			.map_err(|x| io::Error::new(io::ErrorKind::Other, x))?;

		self.stream.write(&(data.len() as u64).to_be_bytes())?;

		self.stream.write_all(&data)
	}

	pub fn recv(&mut self) -> io::Result<T> {
		let mut length_bytes = [0u8; 8];
		self.stream.read_exact(&mut length_bytes[..])?;
		let data_length = u64::from_be_bytes(length_bytes);

		let mut data = vec![0u8; data_length as usize];

		let bytes_read = self.stream.read(&mut data[..])?;

		bincode::DefaultOptions::new()
			.deserialize(&data[..bytes_read])
			.map_err(|x| io::Error::new(io::ErrorKind::Other, x))
	}
}

pub struct LocalListener<T>
where
	T: Serialize + DeserializeOwned,
{
	listener: UnixListener,
	path: PathBuf,
	_p: PhantomData<T>,
}

#[allow(dead_code)]
impl<T> LocalListener<T>
where
	T: Serialize + DeserializeOwned,
{
	pub fn new<P: AsRef<Path>>(
		path: P,
		mode: u32,
	) -> io::Result<Self> {
		let _ = fs::remove_file(&path);

		let listener = UnixListener::bind(&path)?;

		fs::set_permissions(&path, fs::Permissions::from_mode(mode))?;

		Ok(Self {
			path: path.as_ref().to_path_buf(),
			listener,
			_p: PhantomData,
		})
	}

	pub fn accept(
		&self,
	) -> io::Result<(Channel<T, UnixStream>, SocketAddr)> {
		let (stream, addr) = self.listener.accept()?;

		Ok((Channel::new(stream), addr))
	}
}

impl<T> std::ops::Deref for LocalListener<T>
where
	T: Serialize + DeserializeOwned,
{
	type Target = UnixListener;

	fn deref(&self) -> &Self::Target {
		&self.listener
	}
}

impl<T> std::ops::DerefMut for LocalListener<T>
where
	T: Serialize + DeserializeOwned,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.listener
	}
}

impl<T> Drop for LocalListener<T>
where
	T: Serialize + DeserializeOwned,
{
	fn drop(&mut self) {
		let _ = fs::remove_file(self.path.as_path());
	}
}

#[allow(dead_code)]
pub fn connect<T, P: AsRef<Path>>(
	path: P,
) -> io::Result<Channel<T, UnixStream>>
where
	T: Serialize + DeserializeOwned,
{
	Ok(Channel::new(UnixStream::connect(path)?))
}

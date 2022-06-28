use std::collections::HashMap;
use std::ffi::OsStr;

use libloading::{Library, Symbol};

use super::{constants, BoxedRuntime, Runtime};

use crate::error::ZeusError;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
	Unloadable(libloading::Error),
	SymbolLookup(libloading::Error),
	RuntimeNotLoaded,
	RuntimeAlreadyLoaded,
	RuntimeInitError(super::Error),
	IncompatibleRuntimeApi,
}

impl std::fmt::Display for Error {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		use Error::*;

		match self {
			Unloadable(e) => write!(f, "cannot load runtime: {}", e),
			SymbolLookup(e) => {
				write!(f, "cannot find runtime constructor: {}", e)
			},
			RuntimeNotLoaded => write!(f, "runtime is not loaded"),
			RuntimeAlreadyLoaded => {
				write!(f, "runtime is already loaded")
			},
			RuntimeInitError(e) => write!(f, "runtime error: {}", e),
			IncompatibleRuntimeApi => write!(
				f,
				"runtime is incompatible with current version"
			),
		}
	}
}

impl std::error::Error for Error {}

impl From<Error> for ZeusError {
	fn from(e: Error) -> Self {
		Self {
			caller: "RuntimeManager".to_string(),
			message: e.to_string(),
		}
	}
}

#[allow(dead_code)]
pub struct RuntimeLibrary {
	runtime: BoxedRuntime,
	library: Library,
}

impl std::fmt::Debug for RuntimeLibrary {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		f.debug_struct("RuntimeLibrary")
			.field("name", &self.runtime.name())
			.field("version", &self.runtime.version())
			.finish()
	}
}

impl Drop for RuntimeLibrary {
	fn drop(&mut self) {
		self.runtime.exit();
	}
}

#[derive(Debug, Default)]
pub struct RuntimeManager {
	pub runtimes: HashMap<&'static str, RuntimeLibrary>,
}

#[allow(dead_code)]
impl RuntimeManager {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn load<P: AsRef<OsStr>>(
		&mut self,
		path: P,
	) -> Result<&mut BoxedRuntime, Error> {
		unsafe {
			let library = match Library::new(path) {
				Ok(lib) => lib,
				Err(e) => return Err(Error::Unloadable(e)),
			};

			let constructor: Symbol<
				constants::RuntimeConstructorSymbol,
			> = match library.get(
				constants::RUNTIME_CONSTRUCTOR_SYMBOL_NAME.as_bytes(),
			) {
				Ok(symbol) => symbol,
				Err(e) => return Err(Error::SymbolLookup(e)),
			};

			let mut runtime = Box::from_raw(constructor());
			let runtime_name = runtime.name();

			if runtime.rt_api_version()
				!= constants::SUPPORTED_RT_API_VERSION
			{
				return Err(Error::IncompatibleRuntimeApi);
			}

			if self.is_loaded(runtime_name) {
				return Err(Error::RuntimeAlreadyLoaded);
			}

			match runtime.init() {
				Ok(_) => {},
				Err(e) => return Err(Error::RuntimeInitError(e)),
			};

			self.runtimes.insert(
				runtime_name,
				RuntimeLibrary { runtime, library },
			);

			let runtime = &mut self
				.runtimes
				.get_mut(runtime_name)
				.unwrap()
				.runtime;

			Ok(runtime)
		}
	}

	pub fn unload(
		&mut self,
		runtime_name: &str,
	) -> Result<(), Error> {
		if !self.is_loaded(runtime_name) {
			return Err(Error::RuntimeNotLoaded);
		}

		self.runtimes.remove(runtime_name).unwrap();

		Ok(())
	}

	pub fn unload_all(&mut self) {
		for _ in self.runtimes.drain() {}
	}

	pub fn is_loaded(&self, runtime_name: &str) -> bool {
		self.runtimes.contains_key(runtime_name)
	}

	pub fn get(&self, runtime_name: &str) -> Option<&Runtime> {
		self.runtimes.get(runtime_name).map(|x| x.runtime.as_ref())
	}

	pub fn get_mut(
		&mut self,
		runtime_name: &str,
	) -> Option<&mut Runtime> {
		self.runtimes
			.get_mut(runtime_name)
			.map(|x| x.runtime.as_mut())
	}
}

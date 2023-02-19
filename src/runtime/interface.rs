pub use crate::config::GlobalConfig;
pub use crate::term::Terminal;

#[derive(Debug, thiserror::Error)]
#[error("{description}")]
pub struct Error {
	description: String,
}

impl Error {
	pub fn new(description: String) -> Self {
		Self { description }
	}
}

pub type Result<T> = std::result::Result<T, Error>;

/// A trait describing a common interface for all Runtimes.
///
/// **Important:** _All_ trait methods _must_ block the current thread until said operation has been completed.
///
/// While the thread is blocked by one such operation, it is considered
/// bad practice to write to stdout or stderr as the runtime should have
/// taken control of them to perform logging of its own. Violating this
/// might, and probably will, lead to malformed output on the screen,
/// which is definitely not what we want.
///
/// Each runtime is responsible for its own resources, which means it is
/// responsible for performing a cleanup when a resource is no longer needed.
/// Failure to do so will result in the usual bugs such as memory leaks and
/// zombie threads. It is also responsible for not modifying the global state
/// of the process, this includes, but is not limited to, working directory,
/// umask, signal handlers, etc.
pub trait IRuntime {
	fn name(&self) -> &'static str;
	fn version(&self) -> &'static str;

	/// Called when the runtime is first loaded.
	fn init(
		&mut self,
		config: &GlobalConfig,
		term: &mut Terminal,
	) -> Result<()>;
	/// Called when the runtime is unloaded.
	fn exit(&mut self);

	/// Create/update base image
	fn create_image(
		&mut self,
		config: &GlobalConfig,
		term: &mut Terminal,
	) -> Result<()>;

	/// Create  a new machine or update an existing one.
	fn create_machine(
		&mut self,
		config: &GlobalConfig,
		term: &mut Terminal,
	) -> Result<()>;

	/// Start and attach machine to stdin, stdout, stderr.
	fn start_machine(
		&mut self,
		config: &GlobalConfig,
		term: &mut Terminal,
	) -> Result<()>;
}

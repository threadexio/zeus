/// Set the maximum level that will be logged.
#[macro_export]
macro_rules! set_log_level {
	($level:expr) => {
		#[allow(unused_unsafe)]
		unsafe {
			$crate::log::__private_log::MAX_LOG_LEVEL = $level;
		}
	};
}
pub use set_log_level;

/// Don't use me. Use:
/// - `fatal!`
/// - `error!`
/// - `warn!`
/// - `info!`
/// - `debug!`
/// - `trace!`
#[macro_export]
macro_rules! log {
	(target: $target:expr, $level:expr, $($arg:tt)+) => {
		$crate::log::__private_log::imp_log($level, Some($target), format!($($arg)+))
	};
	($level:expr, $($arg:tt)+) => {
		$crate::log::__private_log::imp_log($level, Option::<&str>::None, format!($($arg)*))
	}
}
pub use log;

/// Log a fatal error
///
/// # Example
///
/// ```
/// use zeus::fatal;
///
/// fatal!(target: "network", "Fatal: {}: {}", 42, "some error");
/// fatal!("Fatal: {}: {}", 42, "some error");
/// ```
#[macro_export]
macro_rules! fatal {
	(target: $target:expr, $($arg:tt)+) => {
		$crate::log!(target: $target, $crate::log::Level::Fatal, $($arg)+)
	};
	($($arg:tt)+) => {
		$crate::log!($crate::log::Level::Fatal, $($arg)+)
	}
}

/// Log an error
///
/// # Example
///
/// ```
/// use zeus::error;
///
/// error!(target: "network", "Error: {}: {}", 42, "some error");
/// error!("Error: {}: {}", 42, "some error");
/// ```
#[macro_export]
macro_rules! error {
	(target: $target:expr, $($arg:tt)+) => {
		$crate::log!(target: $target, $crate::log::Level::Error, $($arg)+)
	};
	($($arg:tt)+) => {
		$crate::log!($crate::log::Level::Error, $($arg)+)
	}
}
pub use error;

/// Log a warning
///
/// # Example
///
/// ```
/// use zeus::warning;
///
/// warning!(target: "network", "Warning: {}: {}", 42, "some warning");
/// warning!("Warning: {}: {}", 42, "some warning");
/// ```
#[macro_export]
macro_rules! warning {
	(target: $target:expr, $($arg:tt)+) => {
		$crate::log!(target: $target, $crate::log::Level::Warn, $($arg)+)
	};
	($($arg:tt)+) => {
		$crate::log!($crate::log::Level::Warn, $($arg)+)
	}
}
pub use warning;

/// Log an informational message
///
/// # Example
///
/// ```
/// use zeus::info;
///
/// info!(target: "network", "Info: {}: {}", 42, "some info");
/// info!("Info: {}: {}", 42, "some info");
/// ```
#[macro_export]
macro_rules! info {
	(target: $target:expr, $($arg:tt)+) => {
		$crate::log!(target: $target, $crate::log::Level::Info, $($arg)+)
	};
	($($arg:tt)+) => {
		$crate::log!($crate::log::Level::Info, $($arg)+)
	}
}
pub use info;

/// Log a debug message
///
/// # Example
///
/// ```
/// use zeus::debug;
///
/// debug!(target: "network", "Debug: {}: {}", 42, "some info");
/// debug!("Debug: {}: {}", 42, "some debug");
/// ```
#[macro_export]
macro_rules! debug {
	(target: $target:expr, $($arg:tt)+) => {
		$crate::log!(target: $target, $crate::log::Level::Debug, $($arg)+)
	};
	($($arg:tt)+) => {
		$crate::log!($crate::log::Level::Debug, $($arg)+)
	}
}
pub use debug;

/// Log a trace message
///
/// # Example
///
/// ```
/// use zeus::trace;
///
/// trace!(target: "network", "Trace: {}: {}", 42, "some trace");
/// trace!("Trace: {}: {}", 42, "some trace");
/// ```
#[macro_export]
macro_rules! trace {
	(target: $target:expr, $($arg:tt)+) => {
		$crate::log!(target: $target, $crate::log::Level::Trace, $($arg)+)
	};
	($($arg:tt)+) => {
		$crate::log!($crate::log::Level::Trace, $($arg)+)
	}
}
pub use trace;

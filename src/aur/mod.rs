//! A standard implementation of a client for the AUR
//! supporting the full specification found on the [official Arch
//! Wiki](https://wiki.archlinux.org/title/Aurweb_RPC_interface).
//!
//! # Example
//! ```rust,ignore
//! let aur = Aur::new("https://aur.archlinux.org/", "my-app/1.0").unwrap();
//!
//! println!(
//!         "{:#?}",
//!         aur.search(By::Name, ["pacman"].iter()).unwrap()
//! );
//! ```
#![allow(dead_code)]

mod cache;
mod client;
mod error;
mod pkg;

pub use error::{Error, Result};

pub use client::{Aur, Response};
pub use pkg::{By, Package};

#[cfg(test)]
mod tests {
	use super::*;

	use std::thread::sleep;
	use std::time::Duration;

	#[test]
	fn test_cache() {
		let mut cache = cache::Cache::<String, String>::new(
			Duration::from_millis(100),
		);

		// add new data
		assert_eq!(
			cache.add(
				"test key".to_string(),
				"test value".to_string()
			),
			None
		);

		// retrieve added data
		assert_eq!(
			cache.get("test key"),
			Some(&"test value".to_string())
		);

		// wait for the data to expire
		sleep(Duration::from_millis(200));

		// check if data has correctly expired
		assert_eq!(cache.get("test key"), None);

		// overwrite data and see if the old expired
		// data is returned
		assert_eq!(
			cache.add("test key".to_string(), "test".to_string()),
			None
		);

		sleep(Duration::from_millis(200));

		assert_eq!(cache.remove("test key"), None);
	}

	#[test]
	fn test_aur() {
		let mut aur =
			Aur::new("https://aur.archlinux.org/", "").unwrap();

		println!(
			"{:#?}",
			aur.search(By::Name, ["pacman"].iter()).unwrap()
		);

		assert!(!aur
			.search(By::Name, ["pacman"].iter())
			.unwrap()
			.is_empty());

		assert!(
			aur.info(["zeus", "zeus-bin"].iter()).unwrap().len() == 2
		);

		assert!(aur
			.info(["zeus", "zeus-bin"].iter())
			.unwrap()
			.iter()
			.all(|x| x.name == "zeus" || x.name == "zeus-bin"));
	}
}

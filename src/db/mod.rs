#![allow(dead_code)]

mod database;
mod lock;
mod pkg;

pub mod tools;

pub use database::Db;
pub use pkg::Package;

#[cfg(test)]
mod tests {
	use super::*;

	use std::path::Path;

	#[test]
	fn test_db() {
		let mut db = Db::new("/tmp");

		assert_eq!(db.get_pkg("zeus-bin"), None);

		let pkg = db
			.add_pkg(
				"zeus-bin",
				"https://aur.archlinux.org/zeus-bin.git",
			)
			.unwrap();

		assert_eq!(pkg.name(), "zeus-bin");
		assert_eq!(pkg.path(), Path::new("/tmp/zeus-bin"));
		assert!(!pkg.get_install_files().unwrap().is_empty());

		assert_eq!(db.get_pkg(".zeus-bin"), None);
		assert_eq!(db.get_pkg("zeus-bin//"), None);
		assert_eq!(db.get_pkg("/../../../zeus-bin"), None);
		assert_eq!(db.get_pkg("/../../../\\/zeus-bin"), None);
		assert_eq!(db.get_pkg("/../../../\\/zeus-bin/../.."), None);
		assert_eq!(db.get_pkg("/../../../\\/zeus-bin../../"), None);

		assert!(db.remove_pkg(pkg).is_ok());

		assert!(!Path::new("/tmp/zeus-bin").exists());
	}
}

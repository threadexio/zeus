use ::std::path::Path;

use super::prelude::*;

pub fn runtime(_: GlobalConfig, config: RuntimeConfig) -> Result<()> {
	if config.list {
		let runtime_dir = Path::new(constants::LIB_DIR)
			.join("runtimes")
			.read_dir()?;

		for entry in runtime_dir {
			let entry = match entry {
				Ok(v) => v,
				Err(_) => continue,
			};

			let entry_name_os = entry.file_name();

			let entry_name = match entry_name_os.to_str() {
				Some(v) => v,
				None => continue,
			};

			if !entry_name.starts_with("librt_")
				|| !entry_name.ends_with(".so")
			{
				continue;
			}

			if !entry.path().is_file() {
				continue;
			}

			debug!("Test-loading runtime {}", entry_name);

			let rt = match Runtime::load(&entry.path()) {
				Ok(v) => v,
				Err(e) => {
					error!(
						"Runtime {} cannot be loaded: {}",
						entry_name, e
					);
					continue;
				},
			};

			info!(
				"{} - {} v{}",
				entry_name,
				rt.name().bold(),
				rt.version().yellow(),
			);
		}
	}

	Ok(())
}

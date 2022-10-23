use super::prelude::*;

use colored::Colorize;

use std::fs::read_dir;

pub fn runtime(
	gopts: GlobalOptions,
	opts: RuntimeOptions,
) -> Result<()> {
	if opts.list {
		let runtime_dir = read_dir(&gopts.runtime_dir)?;

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

			// TODO: Don't run Runtime::init() here

			let rt = match Runtime::load(&entry.path(), &gopts) {
				Ok(v) => v,
				Err(e) => {
					warn!(
						"Runtime {} cannot be loaded: {}",
						entry_name, e
					);
					continue;
				},
			};

			info!(
				"{} - {} v{} (RT_API v{})",
				entry_name,
				rt.name().bold(),
				rt.version().yellow(),
				rt.rt_api_version()
			);
		}
	}

	Ok(())
}

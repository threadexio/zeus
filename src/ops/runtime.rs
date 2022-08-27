use super::prelude::*;

use std::fs::read_dir;

pub fn runtime(
	term: &mut Terminal,
	rt_manager: &mut RuntimeManager,
	cfg: Config,
	opts: &mut RuntimeOptions,
) -> Result<()> {
	if opts.list {
		let runtime_dir = read_dir(&cfg.runtime_dir)?;

		let mut working_runtimes: Vec<String> = Vec::new();

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
			unsafe {
				let rtlib =
					match rt_manager._load_unchecked(entry.path()) {
						Ok(v) => v,
						Err(e) => {
							warn!(
								"Runtime {} cannot be loaded: {}",
								entry_name, e
							);
							continue;
						},
					};

				working_runtimes.push(format!(
					"{} - {} v{} (RT_API v{})",
					entry_name,
					rtlib.runtime.name().bold(),
					rtlib.runtime.version().yellow(),
					rtlib.runtime.rt_api_version()
				));
			}
		}

		term.list(
			format!(
				"{} working {} found:",
				working_runtimes.len(),
				if working_runtimes.len() == 1 {
					"runtime"
				} else {
					"runtimes"
				}
			),
			working_runtimes.iter(),
			1,
		)?;
	}

	Ok(())
}

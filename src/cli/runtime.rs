use ::std::path::Path;

use super::prelude::*;

pub fn runtime(_: GlobalConfig, config: RuntimeConfig) -> Result<()> {
	if config.list {
		let runtime_dir =
			Path::new(constants::LIB_DIR).join("runtimes");

		runtime_dir
			.read_dir()
			.with_context(|| {
				format!(
					"Unable to list runtime directory '{}'",
					runtime_dir.display()
				)
			})?
			.filter_map(|x| x.ok())
			.map(|x| x.path())
			.filter(|x| x.is_file())
			.filter(|x| match x.file_name() {
				Some(x) => {
					let x = x.to_string_lossy();
					x.starts_with("librt_") && x.ends_with(".so")
				},
				None => false,
			})
			.for_each(|path| {
				debug!("Test-loading runtime '{}'", path.display());

				let rt = match Runtime::load(&path) {
					Ok(v) => v,
					Err(e) => {
						error!(
							"Unable to load runtime '{}': {e}",
							path.display()
						);
						return;
					},
				};

				info!(
					"{}: {} v{}",
					path.display(),
					rt.name().bold(),
					rt.version().yellow(),
				);
			});
	}

	Ok(())
}

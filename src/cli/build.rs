use super::prelude::*;

pub(crate) fn build(
	global_config: GlobalConfig,
	_: BuildConfig,
	runtime: &mut Runtime,
) -> Result<()> {
	debug!("Making new image");

	runtime
		.create_image(&global_config)
		.context("Unable to make image")?;

	runtime
		.create_machine(&global_config)
		.context("Unable to create new builder")?;

	Ok(())
}

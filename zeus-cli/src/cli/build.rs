use super::prelude::*;

pub fn build(
	global_config: GlobalConfig,
	_: BuildConfig,
	term: &mut Terminal,
	runtime: &mut Runtime,
) -> Result<()> {
	if !term.confirm("⚒️  Proceed with build?".underline(), true)
	{
		term.writeln("Aborting.".bold());
		return Ok(());
	}

	runtime
		.create_image(&global_config, term)
		.context("Unable to make image")?;

	runtime
		.create_machine(&global_config, term)
		.context("Unable to create new builder")?;

	Ok(())
}

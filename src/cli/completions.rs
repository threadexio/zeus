use super::prelude::*;

pub(crate) fn completions(
	_: GlobalConfig,
	config: CompletionsConfig,
) -> Result<()> {
	clap_complete::generate(
		clap_complete::Shell::from(config.shell),
		&mut config::command(),
		constants::NAME.to_string(),
		&mut std::io::stdout(),
	);

	Ok(())
}

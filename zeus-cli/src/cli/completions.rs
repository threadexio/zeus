use super::prelude::*;

pub fn completions(
	_: GlobalConfig,
	config: CompletionsConfig,
	term: &mut Terminal,
) -> Result<()> {
	clap_complete::generate(
		clap_complete::Shell::from(config.shell),
		&mut zeus_types::command(),
		constants::NAME.to_string(),
		term.raw_out(),
	);

	Ok(())
}

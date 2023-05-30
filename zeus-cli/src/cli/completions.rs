use super::prelude::*;

pub(crate) fn completions(
	_: GlobalConfig,
	config: CompletionsConfig,
	term: &mut Terminal,
) -> Result<()> {
	clap_complete::generate(
		clap_complete::Shell::from(config.shell),
		&mut zeus_types::command(),
		constants::NAME.to_string(),
		unsafe { term.raw_out() },
	);

	Ok(())
}

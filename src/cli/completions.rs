use super::prelude::*;

pub fn completions(
	_: GlobalConfig,
	opts: CompletionsConfig,
) -> Result<()> {
	clap_complete::generate(
		clap_complete::Shell::from(opts.shell),
		&mut config::command(),
		constants::NAME.to_string(),
		&mut std::io::stdout(),
	);

	Ok(())
}

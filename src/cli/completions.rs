use super::prelude::*;

pub fn completions(opts: CompletionOptions) -> Result<()> {
	clap_complete::generate(
		clap_complete::Shell::from(opts.shell),
		&mut app(),
		constants::NAME.to_string(),
		&mut std::io::stdout(),
	);

	Ok(())
}

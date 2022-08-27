use super::prelude::*;

use clap::CommandFactory;

pub fn completions(opts: &mut CompletionOptions) -> Result<()> {
	if let Some(shell) = opts.shell.as_ref() {
		clap_complete::generate(
			clap_complete::Shell::from(shell.clone()),
			&mut Config::command(),
			constants::NAME.to_string(),
			&mut std::io::stdout(),
		);
	}

	Ok(())
}

use std::io::stdout;

use crate::cli;

use crate::ops::prelude::*;

pub async fn completions(args: &ArgMatches) -> Result<()> {
	if args.is_present("shell") {
		cli::make_completions(
			args.value_of_t::<cli::Shell>("shell")
				.unwrap_or(cli::Shell::Bash),
			&mut stdout(),
		);
	}

	Ok(())
}
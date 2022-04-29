use crate::cli;
use crate::error::ZeusError;

use std::io::stdout;

use clap::ArgMatches;

pub fn misc(args: &ArgMatches) -> Result<(), ZeusError> {
	if args.is_present("shell") {
		cli::make_completions(
			args.value_of_t::<cli::Shell>("shell")
				.unwrap_or(cli::Shell::Bash),
			&mut stdout(),
		);
	}

	Ok(())
}

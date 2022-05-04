use crate::cli;
use crate::error::ZeusError;

use clap::ArgMatches;

use std::io::stdout;

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

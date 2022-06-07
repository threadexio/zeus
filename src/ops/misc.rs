use crate::cli;
use crate::config;
use crate::error::Result;
use crate::log;

use clap::ArgMatches;

use std::io::stdout;

pub async fn misc(
	logger: &mut log::Logger,
	cfg: &mut config::AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	if args.is_present("shell") {
		cli::make_completions(
			args.value_of_t::<cli::Shell>("shell")
				.unwrap_or(cli::Shell::Bash),
			&mut stdout(),
		);
	}

	Ok(())
}

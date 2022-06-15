use std::io::stdout;

use crate::cli;

use crate::ops::prelude::*;

pub async fn completions(
	_logger: &Logger,
	_cfg: &mut config::AppConfig,
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

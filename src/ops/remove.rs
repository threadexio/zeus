use crate::ops::prelude::*;

pub async fn remove(
	logger: &Logger,
	docker: Docker,
	cfg: &mut config::AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	cfg.packages = args
		.values_of("packages")
		.unwrap_or_default()
		.map(|x| x.to_owned())
		.collect();

	if cfg.packages.is_empty() {
		return Err(ZeusError::new(
			"zeus".to_owned(),
			"No packages specified.".to_owned(),
		));
	}

	log_info!(
		logger,
		"zeus",
		"Removing: {}",
		cfg.packages
			.iter()
			.map(|x| x.as_str())
			.collect::<Vec<&str>>()
			.join(" ")
	);

	cfg.remove = true;

	// TODO: Start container and instruct to remove package

	Ok(())
}

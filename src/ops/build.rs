use crate::ops::prelude::*;

pub fn build(
	term: &Terminal,
	runtime: &mut Runtime,
	cfg: &mut AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	cfg.image = args.value_of("image").unwrap().to_owned();
	cfg.machine = args.value_of("name").unwrap().to_owned();

	// remove old machine

	// update image

	// create new machine

	todo!()
}

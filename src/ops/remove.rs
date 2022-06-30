use crate::machine::BoxedMachine;
use crate::message::Message;
use crate::ops::prelude::*;
use crate::unix::LocalListener;

use std::path::Path;

pub fn remove(
	term: &mut Terminal,
	runtime: &mut Runtime,
	mut cfg: AppConfig,
	args: &ArgMatches,
) -> Result<()> {
	cfg.machine = args.value_of("name").unwrap().to_owned();

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

	debug!(term.log, "post-op config", "{:?}", &cfg);

	term.list(
		"The following packages will be REMOVED:",
		cfg.packages.iter(),
		4,
	)?;

	if !term.yes_no_question(
		"Are you sure you want to remove these packages?",
		true,
	)? {
		error!(term.log, "zeus", "Aborting...");
		return Ok(());
	}

	let mut machine: Option<BoxedMachine> = None;
	for m in runtime.list_machines()? {
		if m.name() == cfg.machine {
			machine = Some(m);
			break;
		}
	}

	if machine.is_none() {
		return Err(ZeusError::new(
			"zeus".to_owned(),
			format!("Cannot find builder machine {}", cfg.machine),
		));
	}

	let socket_path = format!("{}/zeus.sock", &cfg.build_dir);
	let listener = zerr!(
		LocalListener::new(Path::new(&socket_path), 0o666),
		"unix",
		"Cannot listen on socket {}",
		&socket_path
	);

	info!(term.log, "zeus", "Starting builder...");

	runtime.start_machine(machine.as_ref().unwrap().as_ref())?;

	runtime.attach_machine(machine.as_ref().unwrap().as_ref())?;

	let (mut channel, _) = zerr!(
		listener.accept(),
		"unix",
		"Cannot open communication stream with builder"
	);

	channel.send(Message::Config(cfg))?;

	loop {
		match channel.recv()? {
			Message::Done => break,
			_ => {},
		}
	}

	Ok(())
}

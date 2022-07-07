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
		format!(
			"The following packages will be {}:",
			"REMOVED".bold()
		),
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

	debug!(term.log, "MachineManager", "Attaching to builder...");

	runtime.attach_machine(machine.as_ref().unwrap().as_ref())?;

	debug!(term.log, "unix", "Waiting for builder to connect...");

	let (mut channel, _) = zerr!(
		listener.accept(),
		"unix",
		"Cannot open communication stream with builder"
	);

	debug!(term.log, "zeus", "Sending config to builder...");

	channel.send(Message::Config(cfg))?;

	debug!(term.log, "zeus", "Entering main event loop...");

	loop {
		match channel.recv()? {
			Message::Success(pkgs) => {
				println!("{} Removed packages:", "=>".green().bold(),);
				for pkg in pkgs {
					println!("    {}", pkg.bold())
				}
				return Ok(());
			},
			Message::Failure(error) => {
				return Err(ZeusError::new(
					"builder".to_string(),
					error,
				))
			},
			_ => {},
		}
	}
}

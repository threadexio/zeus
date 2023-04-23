use std::collections::HashMap;
use std::env::{self, Args};
use std::fs;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Output, Stdio};

use anyhow::{bail, Context, Result};

fn cargo<F>(prepare: F) -> Result<Output>
where
	F: FnOnce(&mut Command) -> &mut Command,
{
	prepare(
		Command::new("cargo")
			.stdin(Stdio::inherit())
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit()),
	)
	.output()
	.context("failed to execute cargo")
}

fn cargo_metadata() -> Result<serde_json::Value> {
	let c = cargo(|c| {
		c.stdout(Stdio::piped())
			.arg("metadata")
			.arg("--format-version=1")
	})?;

	if !c.status.success() {
		bail!(
			"failed to get cargo metadata: exited with: {}",
			c.status.code().unwrap_or(-1)
		);
	}
	let output = String::from_utf8(c.stdout)
		.context("cargo metadata did not emit valid utf8")?;

	serde_json::from_str::<serde_json::Value>(&output)
		.context("failed to parse cargo metadata json")
}

fn clean() -> Result<()> {
	cargo(|c| c.arg("clean"))?;
	if let Err(e) = fs::remove_file("./build") {
		if e.kind() != io::ErrorKind::NotFound {
			return Err(e)
				.context("failed to remove link to build directory");
		}
	}

	Ok(())
}

fn completions() -> Result<()> {
	let mut shells: HashMap<String, PathBuf> = HashMap::new();
	shells.insert(
		"bash".into(),
		"overlay/usr/share/bash-completion/completions/zeus".into(),
	);
	shells.insert(
		"fish".into(),
		"overlay/usr/share/fish/vendor_completions.d/zeus.fish"
			.into(),
	);
	shells.insert(
		"zsh".into(),
		"overlay/usr/share/zsh/site-functions/_zeus".into(),
	);

	for (shell, out_path) in shells {
		let output = cargo(|c| {
			c.stdout(Stdio::piped())
				.args([
					"run",
					"--bin=zeus",
					"-q",
					"--",
					"--config=/dev/null",
					"--build-dir=.",
					"completions",
					"-s",
				])
				.arg(&shell)
		})?;
		if !output.status.success() {
			bail!("failed to generate completions for {}: zeus exited with {}", &shell, output.status.code().unwrap_or(-1))
		}

		let mut out = fs::File::options()
			.write(true)
			.truncate(true)
			.create(true)
			.open(out_path)
			.context("failed to open completions output file")?;

		out.write_all(&output.stdout)
			.context("cannot write completions to output file")?;
	}

	Ok(())
}

fn check_tool_exists(tool: &str) -> Result<bool> {
	if let Err(e) = Command::new(tool).output() {
		if e.kind() == io::ErrorKind::NotFound {
			Ok(false)
		} else {
			Err(e)
				.with_context(|| format!("failed to execute {tool}"))
		}
	} else {
		Ok(true)
	}
}

fn install(destdir: &Path, profile: &str) -> Result<()> {
	fn turboinstall<F>(prepare: F) -> Result<Output>
	where
		F: FnOnce(&mut Command) -> &mut Command,
	{
		prepare(
			Command::new("turboinstall")
				.stdin(Stdio::inherit())
				.stdout(Stdio::inherit())
				.stderr(Stdio::inherit()),
		)
		.output()
		.context("failed to execute turboinstall")
	}

	let profile_path = Path::new("profiles")
		.join(format!("{profile}.json"))
		.canonicalize()
		.context("failed to find profile")?;

	let destdir =
		destdir.canonicalize().context("failed to find destdir")?;

	if !turboinstall(|c| {
		c.arg("--profile")
			.arg(&profile_path)
			.arg("--")
			.arg(&destdir)
			.arg("./overlay")
	})?
	.status
	.success()
	{
		bail!("failed to install primary overlay");
	}

	for overlay in walkdir::WalkDir::new("runtimes")
		.max_depth(2)
		.follow_links(false)
		.contents_first(false)
		.into_iter()
		.filter_map(|x| x.ok())
		.filter(|x| x.file_name().to_string_lossy() == "overlay")
		.map(|x| x.path().to_path_buf())
	{
		if !turboinstall(|c| {
			c.arg("--profile")
				.arg(&profile_path)
				.arg("--")
				.arg(&destdir)
				.arg(&overlay)
		})?
		.status
		.success()
		{
			bail!("failed to install runtime overlay `{overlay:?}`")
		}
	}

	Ok(())
}

fn install_command_wrapper(args: &mut Args) -> Result<()> {
	let destdir = args.next().context("expected destdir")?;
	let profile = args.next().context("expected profile")?;
	install(Path::new(&destdir), &profile)
}

fn package_arch(out: &Path) -> Result<()> {
	if !check_tool_exists("makepkg")? {
		bail!("makepkg was not found. arch packages can only be built in arch-based systems");
	}

	let arch_out = out.join("arch");
	fs::create_dir_all(&arch_out)
		.context("failed to create arch package work directory")?;

	fs::copy("pkg/arch/PKGBUILD", arch_out.join("PKGBUILD"))
		.context("failed to copy PKGBUILD to work directory")?;

	let repo_path =
		env::current_dir().context("failed to get current dir")?;

	env::set_current_dir(&arch_out)
		.context("failed to move into package work directory")?;

	let r = Command::new("makepkg")
		.stdin(Stdio::inherit())
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.env("PKGDEST", out)
		.env("_zeus_repo", repo_path)
		.arg("--force")
		.arg("--cleanbuild")
		.output()
		.context("failed to execute makepkg")?;

	if !r.status.success() {
		bail!(
			"failed to build arch package: makepkg exited with: {}",
			r.status.code().unwrap_or(-1)
		);
	}

	Ok(())
}

fn package(args: &mut Args) -> Result<()> {
	if !check_tool_exists("turboinstall")? {
		let install_succeeded =
			cargo(|c| c.args(["install", "turboinstall"]))?
				.status
				.success();

		if !install_succeeded {
			bail!("failed to install turboinstall with cargo. Refusing to continue...");
		}
	}

	let mut out_path = {
		if let Ok(v) = env::var("CARGO_TARGET_DIR") {
			PathBuf::from(v)
		} else if let Some(v) = cargo_metadata()?
			.as_object()
			.and_then(|x| x.get("target_directory"))
			.and_then(|x| x.as_str())
			.map(|x| x.to_string())
		{
			PathBuf::from(v)
		} else {
			eprintln!("warning: using hardcoded target path");
			PathBuf::from("target")
		}
	};
	out_path.push("package");
	fs::create_dir_all(&out_path)
		.context("failed to create package out dir")?;

	match args.next() {
		Some(v) => match v.as_str() {
			"arch" => package_arch(&out_path),
			_ => bail!("Unknown package type: {v}"),
		},
		None => {
			package_arch(&out_path)?;

			Ok(())
		},
	}
}

fn ci_flow() -> Result<()> {
	if !cargo(|c| c.args(["build", "--workspace"]))?
		.status
		.success()
	{
		bail!("failed to build workspace");
	}

	if !cargo(|c| c.args(["xtask", "package", "arch"]))?
		.status
		.success()
	{
		bail!("failed to package for arch");
	}

	Ok(())
}

fn try_main() -> Result<()> {
	let mut args = env::args();
	let _ = args.next();
	let command = match args.next() {
		Some(v) => v,
		None => {
			eprintln!("Usage: cargo xtask [target]");
			eprintln!();
			print_help();
			return Ok(());
		},
	};

	match command.as_str() {
		"distclean" => clean(),
		"compgen" => completions(),
		"install" => install_command_wrapper(&mut args),
		"package" => package(&mut args),
		"ci" => ci_flow(),
		_ => {
			eprintln!("Error: Unknown target: {command}");
			print_help();
			Ok(())
		},
	}
}

fn main() {
	if let Err(e) = try_main() {
		eprintln!("{e:?}");
		process::exit(1);
	}
}

fn print_help() {
	eprintln!(
		"\
Targets:
--------
  distclean                           - Clean all build artifacts
  compgen [shell]                     - Generate shell completions
                                          [shell]: one of: bash, fish, zsh
  install [destdir] [profile]         - Install the package
                                          [destdir]: install root (path)
                                          [profile]: one of the filenames inside `profiles/` without the extension
  package [type]                      - Create a package
                                          [type]: one of: arch
  ci                                  - Run the ci workflow
"
	);
}

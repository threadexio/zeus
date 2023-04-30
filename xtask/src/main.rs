use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{bail, Context, Result};

mod tools;
use tools::*;

const TARGET_DIR: &str = env!("TARGET_DIR");

fn get_build_root(profile: &str) -> PathBuf {
	Path::new(TARGET_DIR).join(match profile {
		"dev" => "debug",
		p => p,
	})
}

fn completions() -> Result<()> {
	for (shell, out_path) in [
		(
			"bash",
			"overlay/usr/share/bash-completion/completions/zeus",
		),
		(
			"fish",
			"overlay/usr/share/fish/vendor_completions.d/zeus.fish",
		),
		("zsh", "overlay/usr/share/zsh/site-functions/_zeus"),
	] {
		let output = Cargo::new()
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
			.arg(shell)
			.with_stdout()?;

		if !output.status.success() {
			bail!("failed to generate completions for {}: zeus exited with {}", &shell, output.status.code().unwrap_or(-1));
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

fn install(
	destdir: &Path,
	profile: &str,
	fakeroot_save_file: Option<&Path>,
) -> Result<()> {
	let profile_path = Path::new("profiles")
		.join(format!("{profile}.json"))
		.canonicalize()
		.context("failed to find profile")?;

	let destdir =
		destdir.canonicalize().context("failed to find destdir")?;

	let build_root = get_build_root(profile);
	env::set_var("BUILD_ROOT", build_root);

	let mut c = Turboinstall::new();
	c.arg("--profile")
		.arg(&profile_path)
		.arg("--")
		.arg(&destdir)
		.arg("./overlay");

	let mut c = match fakeroot_save_file {
		None => c.into_inner(),
		Some(_) => {
			let mut fakeroot = Fakeroot::new(fakeroot_save_file);
			fakeroot.arg("--").embed_command(&c);
			fakeroot.into_inner()
		},
	};

	if !c.output()?.status.success() {
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
		let mut c = Turboinstall::new();
		c.arg("--profile")
			.arg(&profile_path)
			.arg("--")
			.arg(&destdir)
			.arg(&overlay);

		let mut c = match fakeroot_save_file {
			None => c.into_inner(),
			Some(_) => {
				let mut fakeroot = Fakeroot::new(fakeroot_save_file);
				fakeroot.arg("--").embed_command(&c);
				fakeroot.into_inner()
			},
		};

		if !c.output()?.status.success() {
			bail!("failed to install runtime overlay `{overlay:?}`")
		}
	}

	Ok(())
}

fn package(package_type: &str, profile: &str) -> Result<()> {
	fn package_tar(out: &Path, profile: &str) -> Result<()> {
		if !Fakeroot::exists() {
			bail!("fakeroot was not found: tar packages can only be built with fakeroot");
		}

		let tar_out = out.join("tar");
		fs::create_dir_all(&tar_out)
			.context("failed to create tar package work directory")?;

		let tar_root = tar_out.join("root");
		fs::create_dir_all(&tar_root)
			.context("failed to create tar package root directory")?;

		let fakeroot_save_file = tar_out.join("fakeroot.save");
		if let Err(e) = fs::remove_file(&fakeroot_save_file) {
			if e.kind() != io::ErrorKind::NotFound {
				return Err(e).context(
					"failed to remove fakeroot save from previous build",
				);
			}
		}

		install(&tar_root, profile, Some(&fakeroot_save_file))
			.context("failed to install to tar root")?;

		if !Fakeroot::new(Some(&fakeroot_save_file))
			.arg("--")
			.embed_command(
				Tar::new()
					.arg("--auto-compress")
					.arg("--create")
					.arg("--verbose")
					.arg("--preserve-permissions")
					.arg("--file")
					.arg(out.join("zeus-bin.tar.gz"))
					.arg("--directory")
					.arg(tar_root)
					.arg("--no-acls")
					.arg("--no-selinux")
					.arg("--no-xattrs")
					.arg("--")
					.arg("."),
			)
			.output()?
			.status
			.success()
		{
			bail!("failed to archive tar root");
		}

		Ok(())
	}

	fn package_arch(out: &Path, profile: &str) -> Result<()> {
		if !Makepkg::exists() {
			bail!("makepkg was not found: arch packages can only be built in arch-based systems");
		}

		let arch_out = out.join("arch");
		fs::create_dir_all(&arch_out).context(
			"failed to create arch package work directory",
		)?;

		fs::copy("pkg/arch/PKGBUILD", arch_out.join("PKGBUILD"))
			.context("failed to copy PKGBUILD to work directory")?;

		let repo_path = env::current_dir()
			.context("failed to get current dir")?;

		env::set_current_dir(&arch_out)
			.context("failed to move into package work directory")?;

		let r = Makepkg::new()
			.env("PKGDEST", out)
			.env("_zeus_repo", repo_path)
			.env("_zeus_profile", profile)
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

	if !Turboinstall::exists() {
		let install_succeeded = Cargo::new()
			.args(["install", "turboinstall"])
			.output()?
			.status
			.success();

		if !install_succeeded {
			bail!("failed to install turboinstall with cargo");
		}
	}

	let mut out_path = get_build_root(profile);
	out_path.push("package");
	fs::create_dir_all(&out_path)
		.context("failed to create package out dir")?;

	macro_rules! package_select {
		($ptype:expr => ( $arg1:expr, $arg2:expr ) {
			$($type:expr => $fn:ident)*
		}) => {
			match $ptype {
				$(
					$type => $fn($arg1, $arg2),
				)*
				"all" => {
					$(
						$fn($arg1, $arg2)?;
					)*

					Ok(())
				}
				_ => bail!("unknown package type"),
			}
		};
	}

	package_select!(
		package_type => (&out_path, profile) {
			"tar" => package_tar
			"arch" => package_arch
		}
	)
}

fn ci_flow() -> Result<()> {
	if !Cargo::new()
		.arg("build")
		.arg("--workspace")
		.arg("--profile")
		.arg("release")
		.output()?
		.status
		.success()
	{
		bail!("failed to build workspace");
	}

	package("all", "release")?;

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
		"compgen" => completions(),
		"install" => {
			let destdir = args.next().context("expected destdir")?;
			let profile = args.next().context("expected profile")?;
			install(Path::new(&destdir), &profile, None)
		},
		"package" => {
			let package_type =
				args.next().context("expected package type")?;
			let profile = args.next().context("expected profile")?;
			package(&package_type, &profile)
		},
		"ci" => ci_flow(),

		"help" => {
			print_help();
			Ok(())
		},
		_ => {
			eprintln!("Error: Unknown target: {command}");
			print_help();
			Ok(())
		},
	}
}

fn main() -> ExitCode {
	if let Err(e) = try_main() {
		eprintln!("{e:?}");
		ExitCode::FAILURE
	} else {
		ExitCode::SUCCESS
	}
}

fn print_help() {
	eprintln!(
		"\
Targets:
--------
  compgen [shell]                     - Generate shell completions
                                          [shell]: one of: bash, fish, zsh
  install [destdir] [profile]         - Install the package
                                          [destdir]: install root (path)
  package [type] [profile]            - Create a package
                                          [type]: one of: arch, tar
  ci                                  - Run the CI flow

Arguments:
----------
  [profile]: cargo profile, must also have a definition inside `profiles/`
"
	);
}

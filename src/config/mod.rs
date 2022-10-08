use std::path::PathBuf;

pub mod constants;
use constants::*;

use crate::aur;

use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, ValueEnum)]
pub enum Color {
	Auto,
	Never,
	Always,
}

#[derive(Debug, Clone, Serialize, Deserialize, ValueEnum)]
pub enum Shell {
	Bash,
	Fish,
	Zsh,
}

impl From<Shell> for clap_complete::Shell {
	fn from(s: Shell) -> Self {
		match s {
			Shell::Bash => Self::Bash,
			Shell::Fish => Self::Fish,
			Shell::Zsh => Self::Zsh,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, Args)]
pub struct SyncOptions {
	#[clap(short, long, help = "Upgrade packages")]
	pub upgrade: bool,

	#[clap(long, help = "Install packages after build")]
	pub install: bool,

	#[clap(long, help = "Extra arguments for makepkg")]
	pub build_args: Vec<String>,

	#[clap(
		long = "name",
		help = "Builder machine name",
		default_value = BUILDER_NAME
	)]
	pub machine_name: String,

	pub packages: Vec<aur::Package>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Args)]
pub struct RemoveOptions {
	#[clap(long, help = "Uninstall packages after remove")]
	pub uninstall: bool,

	#[clap(
		long = "name",
		help = "Builder machine name",
		default_value = BUILDER_NAME
	)]
	pub machine_name: String,

	pub packages: Vec<aur::Package>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Args)]
pub struct BuildOptions {
	#[clap(
		long = "name",
		help = "Builder machine name",
		default_value = BUILDER_NAME
	)]
	pub machine_name: String,

	#[clap(
		long = "image",
		help = "Builder machine image",
		default_value = BUILDER_IMAGE
	)]
	pub machine_image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Args)]
pub struct QueryOptions {
	#[clap(
		short,
		long,
		help = "Display additional information on results",
		conflicts_with = "by"
	)]
	pub info: bool,

	#[clap(
		long,
		help = "Query AUR packages by",
		default_value = "name-desc",
		conflicts_with = "info"
	)]
	pub by: aur::By,

	#[clap(
		short,
		long,
		help = "Output format",
		default_value = "pretty"
	)]
	pub output: aur::Output,

	pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Args)]
pub struct CompletionOptions {
	#[clap(
		short,
		long,
		help = "Specify shell to generate completions for"
	)]
	pub shell: Option<Shell>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Args)]
pub struct RuntimeOptions {
	#[clap(short, long, help = "List all available runtimes")]
	pub list: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Subcommand)]
pub enum Operation {
	#[clap(name = "sync", about = "Sync packages", short_flag = 'S')]
	Sync(SyncOptions),
	#[clap(
		name = "remove",
		about = "Remove packages",
		short_flag = 'R'
	)]
	Remove(RemoveOptions),
	#[clap(
		name = "build",
		about = "Build/Update builder",
		short_flag = 'B'
	)]
	Build(BuildOptions),
	#[clap(
		name = "query",
		about = "Query the AUR",
		short_flag = 'Q'
	)]
	Query(QueryOptions),
	#[clap(
		name = "completions",
		about = "Generate shell completions"
	)]
	Completions(CompletionOptions),
	#[clap(name = "runtime", about = "Various runtime operations")]
	Runtime(RuntimeOptions),
}

#[derive(Debug, Clone, Serialize, Deserialize, Parser)]
pub struct GlobalOptions {
	#[clap(
		long,
		help = "Colorize the output",
		default_value = "auto"
	)]
	pub color: Color,

	#[clap(
		short = 'l',
		long = "level",
		help = "Set log level",
		default_value = "info"
	)]
	pub log_level: crate::log::LogLevel,

	#[clap(long = "builddir", help = "Package build directory", default_value = BUILD_DIR)]
	pub build_dir: PathBuf,

	/// Instance to communicate with the AUR RPC interface
	#[clap(long, help = "AUR URL", value_parser = aur::AurValueParser, default_value = AUR_URL)]
	pub aur: aur::Aur,

	/// Name of the runtime to load
	#[clap(long = "rt", help = "Specify runtime to use",  default_value = RUNTIME)]
	pub runtime: String,

	/// Directory to search for runtimes
	#[clap(long = "rtdir", help = "Specify directory to search for runtimes", default_value = RUNTIME_DIR)]
	pub runtime_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, Parser)]
#[clap(name = NAME, version = VERSION, about = DESCRIPTION, long_version = LONG_VERSION)]
pub struct Config {
	#[clap(flatten)]
	pub global_opts: GlobalOptions,

	#[clap(subcommand)]
	pub operation: Operation,
}

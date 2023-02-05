use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Color {
	/// Always use colors
	Always,
	/// Never use colors
	Never,
	/// Automatically decide whether to use colors
	Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Output {
	/// Format the output for humans
	Pretty,
	/// Output JSON
	Json,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

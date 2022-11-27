use ::std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Color {
	Always,
	Never,
	Auto,
}

impl FromStr for Color {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"always" => Ok(Self::Always),
			"never" => Ok(Self::Never),
			"auto" => Ok(Self::Auto),
			_ => Err(()),
		}
	}
}

impl Color {
	pub fn possible_values() -> &'static [&'static str] {
		&["always", "never", "auto"]
	}
}

#[derive(Debug, Clone)]
pub enum Output {
	Pretty,
	Json,
}

impl Output {
	#[allow(dead_code)]
	pub fn possible_values() -> &'static [&'static str] {
		&["pretty", "json"]
	}
}

impl FromStr for Output {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"pretty" => Ok(Self::Pretty),
			"json" => Ok(Self::Json),
			_ => Err(()),
		}
	}
}

#[derive(Debug, Clone)]
pub enum Shell {
	Bash,
	Fish,
	Zsh,
}

impl Shell {
	pub fn possible_values() -> &'static [&'static str] {
		&["bash", "zsh", "fish"]
	}
}

impl FromStr for Shell {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"bash" => Ok(Self::Bash),
			"zsh" => Ok(Self::Zsh),
			"fish" => Ok(Self::Fish),
			_ => Err(()),
		}
	}
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

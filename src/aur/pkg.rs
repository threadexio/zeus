/// Package search types
#[derive(
	Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize,
)]
pub enum By {
	/// Search by package name
	Name,
	/// Search by package name and description
	NameDesc,
	/// Search by maintainer
	Maintainer,
	/// Search by dependencies
	Depends,
	/// Search by dev dependencies
	MakeDepends,
	/// Search by optional dependencies
	OptDepends,
	/// Search by testing dependencies
	CheckDepends,
}

impl By {
	pub(super) fn as_api_repr(&self) -> &'static str {
		match self {
			By::Name => "name",
			By::NameDesc => "name-desc",
			By::Maintainer => "maintainer",
			By::Depends => "depends",
			By::MakeDepends => "makedepends",
			By::OptDepends => "optdepends",
			By::CheckDepends => "checkdepends",
		}
	}

	pub fn possible_values() -> &'static [&'static str] {
		&[
			"name",
			"name-desc",
			"maintainer",
			"depends",
			"makedepends",
			"optdepends",
			"checkdepends",
		]
	}
}

impl ::std::str::FromStr for By {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"name" => Ok(Self::Name),
			"name-desc" => Ok(Self::NameDesc),
			"maintainer" => Ok(Self::Maintainer),
			"depends" => Ok(Self::Depends),
			"makedepends" => Ok(Self::MakeDepends),
			"optdepends" => Ok(Self::OptDepends),
			"checkdepends" => Ok(Self::CheckDepends),
			_ => Err(()),
		}
	}
}

// schema from: https://aur.archlinux.org/rpc/swagger
#[derive(
	Debug,
	Default,
	Clone,
	PartialEq,
	serde::Serialize,
	serde::Deserialize,
)]
pub struct Package {
	#[serde(rename = "ID")]
	pub id: u64,
	#[serde(rename = "Name")]
	pub name: String,
	#[serde(rename = "Description")]
	pub description: Option<String>,

	#[serde(rename = "PackageBaseID")]
	pub package_base_id: u64,
	#[serde(rename = "PackageBase")]
	pub package_base: String,
	#[serde(rename = "Maintainer")]
	pub maintainer: Option<String>,

	#[serde(rename = "NumVotes")]
	pub num_votes: u64,
	#[serde(rename = "Popularity")]
	pub popularity: f32,
	#[serde(rename = "FirstSubmitted")]
	pub first_submitted: u64,

	#[serde(rename = "LastModified")]
	pub last_modified: u64,

	#[serde(rename = "OutOfDate")]
	pub out_of_date: Option<u64>,

	#[serde(rename = "Version")]
	pub version: String,

	#[serde(rename = "URLPath")]
	pub url_path: Option<String>,
	#[serde(rename = "URL")]
	pub url: Option<String>,

	#[serde(rename = "Depends")]
	pub depends: Option<Vec<String>>,

	#[serde(rename = "MakeDepends")]
	pub make_depends: Option<Vec<String>>,

	#[serde(rename = "OptDepends")]
	pub opt_depends: Option<Vec<String>>,

	#[serde(rename = "CheckDepends")]
	pub check_depends: Option<Vec<String>>,

	#[serde(rename = "Conflicts")]
	pub conflicts: Option<Vec<String>>,

	#[serde(rename = "Provides")]
	pub provides: Option<Vec<String>>,

	#[serde(rename = "Replaces")]
	pub replaces: Option<Vec<String>>,

	#[serde(rename = "Groups")]
	pub groups: Option<Vec<String>>,

	#[serde(rename = "License")]
	pub license: Option<Vec<String>>,

	#[serde(rename = "Keywords")]
	pub keywords: Option<Vec<String>>,
}

use ::std::fmt;
impl fmt::Display for Package {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", &self.name)
	}
}

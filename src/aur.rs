use std::fmt;

use clap::ValueEnum;
use const_format::formatcp;
use reqwest::blocking::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};

use crate::config::constants;

const AUR_VERSION: usize = 5;
const AUR_RPC: &'static str =
	const_format::formatcp!("rpc/?v={}", AUR_VERSION);

/// Type alias for request results
pub type AurResult = reqwest::Result<AurResponse>;

/// Package search types
#[derive(
	Debug, Clone, PartialEq, Serialize, Deserialize, ValueEnum,
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

impl fmt::Display for By {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				By::Name => "name".to_owned(),
				By::NameDesc => "name-desc".to_owned(),
				By::Maintainer => "maintainer".to_owned(),
				By::Depends => "depends".to_owned(),
				By::MakeDepends => "makedepends".to_owned(),
				By::OptDepends => "optdepends".to_owned(),
				By::CheckDepends => "checkdepends".to_owned(),
			}
		)
	}
}

#[derive(
	Debug, Clone, PartialEq, Serialize, Deserialize, ValueEnum,
)]
pub enum Output {
	Pretty,
	Json,
}

#[derive(
	Debug, Default, PartialEq, Clone, Serialize, Deserialize,
)]
pub struct Package {
	#[serde(rename = "ID")]
	pub id: u64,
	#[serde(rename = "Name")]
	pub name: String,
	#[serde(rename = "PackageBaseID")]
	pub package_base_id: u64,
	#[serde(rename = "PackageBase")]
	pub package_base: String,
	#[serde(rename = "Version")]
	pub version: String,
	#[serde(rename = "Description")]
	pub description: String,
	#[serde(rename = "URL")]
	pub url: Option<String>,
	#[serde(rename = "NumVotes")]
	pub num_votes: u64,
	#[serde(rename = "Popularity")]
	pub popularity: f32,
	#[serde(rename = "OutOfDate")]
	pub out_of_date: Option<u64>,
	#[serde(rename = "Maintainer")]
	pub maintainer: Option<String>,
	#[serde(rename = "FirstSubmitted")]
	pub first_submitted: u64,
	#[serde(rename = "LastModified")]
	pub last_modified: u64,
	#[serde(rename = "URLPath")]
	pub url_path: Option<String>,

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

impl From<&str> for Package {
	fn from(name: &str) -> Self {
		Self { name: name.to_owned(), ..Default::default() }
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AurResponse {
	#[serde(rename = "resultcount")]
	pub result_count: usize,

	pub results: Vec<Package>,

	#[serde(rename = "type")]
	pub query_type: String,

	pub version: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Aur {
	url: String,
}

impl Aur {
	pub fn new(mut url: String) -> Self {
		if !url.ends_with('/') {
			url.push('/')
		}

		Self { url }
	}

	/// Get full URL of AUR instance
	///
	/// # Example:
	/// ```
	/// let aur_instance = aur::Aur::new().build();
	///
	/// let url = aur_instance.get_url();
	/// ```
	pub fn get_url(&self) -> &str {
		&self.url
	}

	/// Search for packages.
	///
	/// # Example:
	/// ```
	/// let aur_instance = aur::Aur::new().build();
	///
	///	let keywords :HashSet<&str> = HashSet::new();
	/// keywords.insert("zeus");
	/// keywords.insert("zeus-bin");
	///
	/// let response = aur_instance.search(aur::By::Name, &keywords);
	/// ```
	pub fn search<T>(
		&self,
		by: By,
		keywords: impl IntoIterator<Item = T>,
	) -> AurResult
	where
		T: fmt::Display,
	{
		let mut url = format!(
			"{}{}&type=search&by={}",
			&self.url,
			AUR_RPC,
			by.to_string().to_lowercase()
		);

		for keyword in keywords {
			url.push_str(&format!("&arg={}", keyword));
		}

		let res: AurResponse =
			make_req_client().get(url).send()?.json()?;

		Ok(res)
	}

	/// Request package information.
	///
	/// Example:
	/// ```
	/// let aur_instance = aur::Aur::new().build();
	///
	///	let packages :HashSet<&str> = HashSet::new();
	/// packages.insert("zeus");
	/// packages.insert("zeus-bin");
	///
	/// let response = aur_instance.info(&packages);
	/// ```
	pub fn info<T>(
		&self,
		packages: impl IntoIterator<Item = T>,
	) -> AurResult
	where
		T: fmt::Display,
	{
		let mut url = format!("{}{}&type=info", &self.url, AUR_RPC,);

		for package in packages {
			url.push_str(&format!("&arg[]={}", package));
		}

		let res: AurResponse =
			make_req_client().get(url).send()?.json()?;

		Ok(res)
	}
}

fn make_req_client() -> Client {
	ClientBuilder::new()
		.user_agent(formatcp!(
			"{}-{}",
			constants::NAME,
			constants::VERSION
		))
		.build()
		.unwrap()
}

#[derive(Debug, Clone)]
pub struct AurValueParser;
impl clap::builder::TypedValueParser for AurValueParser {
	type Value = Aur;

	fn parse_ref(
		&self,
		cmd: &clap::Command,
		arg: Option<&clap::Arg>,
		value: &std::ffi::OsStr,
	) -> Result<Self::Value, clap::Error> {
		let inner = clap::builder::StringValueParser::new();
		let val = inner.parse_ref(cmd, arg, value)?;

		Ok(Aur::new(val))
	}
}

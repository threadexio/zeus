use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use const_format::formatcp;

use crate::config;

/// Type alias for timestamps
pub type Timestamp = u64;
/// Type alias for id fields
pub type Id = u64;
/// Type alias for version number fields
pub type Version = u8;

/// Type alias for request results
pub type AurResult = reqwest::Result<AurResponse>;

/// Package search types
#[allow(dead_code)]
#[derive(Debug)]
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

#[derive(Debug)]
pub struct AurBuilder {
	host: String,
	protocol: String,

	version: Version,
	rpc_path: String,
}

/// Structure representing an AUR instance
#[derive(Debug)]
pub struct Aur {
	base_url: String,

	client: reqwest::Client,
	headers: reqwest::header::HeaderMap,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
	pub ID: Id,
	pub Name: String,
	pub PackageBaseID: Id,
	pub PackageBase: String,
	pub Version: String,
	pub Description: String,
	pub URL: String,
	pub NumVotes: u64,
	pub Popularity: f32,
	pub OutOfDate: Option<Timestamp>,
	pub Maintainer: Option<String>,
	pub FirstSubmitted: Timestamp,
	pub LastModified: Timestamp,
	pub URLPath: String,

	pub Depends: Option<Vec<String>>,
	pub MakeDepends: Option<Vec<String>>,
	pub OptDepends: Option<Vec<String>>,
	pub CheckDepends: Option<Vec<String>>,
	pub Conflicts: Option<Vec<String>>,
	pub Provides: Option<Vec<String>>,
	pub Replaces: Option<Vec<String>>,
	pub Groups: Option<Vec<String>>,
	pub License: Option<Vec<String>>,
	pub Keywords: Option<Vec<String>>,
}

/// Structure representing the responses
#[derive(Debug, Serialize, Deserialize)]
pub struct AurResponse {
	/// Number of returned packages
	pub resultcount: usize,

	/// Packages returned
	pub results: Vec<Package>,

	/// Query type
	pub r#type: String,
	/// AUR version
	pub version: Version,
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

impl FromStr for By {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"name" => Ok(Self::Name),
			"description" => Ok(Self::NameDesc),
			"maintainer" => Ok(Self::Maintainer),
			"depends" => Ok(Self::Depends),
			"makedepends" => Ok(Self::MakeDepends),
			"optdepends" => Ok(Self::OptDepends),
			"checkdepends" => Ok(Self::CheckDepends),
			_ => unreachable!(),
		}
	}
}

#[allow(dead_code)]
impl AurBuilder {
	/// Create a new AUR instance
	pub fn build(self) -> Aur {
		let client = reqwest::Client::new();

		let mut headers = reqwest::header::HeaderMap::new();
		headers.append(
			reqwest::header::USER_AGENT,
			formatcp!("{}-{}", config::PROGRAM_NAME, config::PROGRAM_VERSION)
				.parse()
				.unwrap(),
		);

		Aur {
			base_url: format!(
				"{}://{}/{}/?v={}",
				self.protocol, self.host, self.rpc_path, self.version
			),
			client: client,
			headers: headers,
		}
	}

	/// Set AUR host
	///
	/// # Example:
	/// ```
	/// let aur_instance = aur::Aur::new()
	///						.host("aur.example.com")
	///						.build();
	/// ```
	pub fn host(mut self, host: String) -> Self {
		self.host = host;
		self
	}

	/// Set AUR protocol
	///
	/// # Example:
	/// ```
	/// let aur_instance = aur::Aur::new()
	///						.protocol("https")
	///						.build();
	/// ```
	pub fn protocol(mut self, protocol: String) -> Self {
		self.protocol = protocol;
		self
	}

	/// Set AUR RPC version
	///
	/// # Example:
	/// ```
	/// let aur_instance = aur::Aur::new()
	///						.version(5)
	///						.build();
	/// ```
	pub fn version(mut self, version: u8) -> Self {
		self.version = version;
		self
	}

	/// Set AUR RPC endpoint path from /
	///
	/// # Example:
	/// ```
	/// let aur_instance = aur::Aur::new()
	///						.rpc_path("rpc/")
	///						.build();
	/// ```
	pub fn rpc_path(mut self, rpc_path: String) -> Self {
		self.rpc_path = rpc_path;
		self
	}
}

#[allow(dead_code)]
impl Aur {
	/// Create a new AurBuilder
	pub fn new() -> AurBuilder {
		AurBuilder {
			host: "aur.archlinux.org".to_owned(),
			protocol: "http".to_owned(),
			rpc_path: "rpc".to_owned(),
			version: 5,
		}
	}

	/// Search for packages.
	///
	/// # Example:
	/// ```
	/// let aur_instance = aur::Aur::new().build();
	///
	/// let response = aur_instance.search(aur::By::Name, vec!["zeus", "zeus-bin"]).await;
	/// ```
	pub async fn search<T>(&self, by: By, keywords: &Vec<T>) -> AurResult
	where
		T: fmt::Display,
	{
		let mut url = format!(
			"{}&type=search&by={}",
			&self.base_url,
			by.to_string().to_lowercase()
		);

		for keyword in keywords {
			url.push_str(&format!("&arg={}", keyword));
		}

		let res: AurResponse = self
			.client
			.get(url)
			.headers(self.headers.clone())
			.send()
			.await?
			.json()
			.await?;

		Ok(res)
	}

	/// Request package information.
	///
	/// Example:
	/// ```
	/// let aur_instance = aur::Aur::new().build();
	///
	/// let response = aur_instance.info(vec!["zeus", "zeus-bin"]).await;
	/// ```
	pub async fn info<T>(&self, packages: &Vec<T>) -> AurResult
	where
		T: fmt::Display,
	{
		let mut url = format!("{}&type=info", &self.base_url);

		for package in packages {
			url.push_str(&format!("&arg[]={}", package));
		}

		let res: AurResponse = self
			.client
			.get(url)
			.headers(self.headers.clone())
			.send()
			.await?
			.json()
			.await?;

		Ok(res)
	}
}

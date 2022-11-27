use std::fmt;
use std::time::Duration;

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use super::cache::Cache;
use super::error::*;
use super::{By, Package};

const AUR_VERSION: usize = 5;
const AUR_RPC_ENDPOINT: &str = "rpc/";

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	#[serde(rename = "resultcount")]
	count: usize,

	results: Vec<Package>,

	#[serde(rename = "type")]
	query_type: String,

	version: usize,

	error: Option<String>,
}

#[derive(Debug, Clone)]

pub struct Aur {
	url: String,
	client: Client,
	cache: Cache<String, Package>,
}

// TODO: URI Size check
//	The specification describes that the longest URI it will accept is 4443 bytes. Implement this please. :)
//		- threadexio

impl Aur {
	/// Create a new [`Aur`] that will communicate with the
	/// server at `url`. This instance will be presented at
	/// the server as `identity`.
	pub fn new(url: &str, identity: &str) -> Result<Self> {
		let url = url.trim().trim_end_matches('/');

		Ok(Self {
			url: url.to_string(),
			client: Client::builder()
				.user_agent(identity.to_string())
				.build()?,
			cache: Cache::new(Duration::from_secs(5 * 60)), // 5 minutes by default
		})
	}

	/// Get the current AUR url.
	///
	/// # Example
	/// ```rust,ignore
	/// let aur = Aur::new("http://aur.example.com", "my-app/1.0").unwrap();
	///
	/// assert_eq!(aur.get_url(), "http://aur.example.com");
	/// ```
	pub fn get_url(&self) -> &str {
		&self.url
	}

	/// Set the current AUR url.
	///
	/// # Example
	/// ```rust,ignore
	/// let aur = Aur::new("http://aur.example.com", "my-app/1.0").unwrap();
	///
	/// aur.url("https://aur.archlinux.org");
	///
	/// assert_eq!(aur.get_url(), "https://aur.archlinux.org");
	/// ```
	pub fn url<U: fmt::Display>(&mut self, url: U) {
		self.url = url.to_string();
	}

	/// Set the cache TTL.
	pub fn ttl(&mut self, ttl: Duration) {
		self.cache.set_ttl(ttl);
	}

	fn prepare_rpc_url(&self) -> String {
		format!("{}/{}?v={}", self.url, AUR_RPC_ENDPOINT, AUR_VERSION)
	}

	fn request(&self, url: &str) -> Result<Response> {
		let response: Response =
			self.client.get(url).send()?.json()?;

		if response.version != AUR_VERSION {
			return Err(Error::InvalidResponse(
				"mismatched versions".to_string(),
			));
		}

		match response.error {
			Some(err) => Err(Error::Query(err)),
			None => Ok(response),
		}
	}

	/// Perform a query on the AUR instance with the
	/// specified keywords returning packages that matched
	/// based on `by`.
	pub fn search<K, I>(
		&mut self,
		by: By,
		keywords: I,
	) -> Result<Vec<Package>>
	where
		K: AsRef<str>,
		I: Iterator<Item = K>,
	{
		let mut url = format!(
			"{}&type=search&by={}",
			self.prepare_rpc_url(),
			by.as_api_repr()
		);

		for keyword in keywords {
			url.push_str(&format!("&arg={}", keyword.as_ref()));
		}

		Ok(self.request(&url)?.results)
	}

	/// Get information about the packages
	/// whose name exists in `packages`.
	pub fn info<P, I>(&mut self, packages: I) -> Result<Vec<Package>>
	where
		P: AsRef<str>,
		I: Iterator<Item = P>,
	{
		let mut pkgs: Vec<String> = vec![];

		let mut params = String::new();
		for package in packages {
			match self.cache.get(package.as_ref()) {
				Some(_) => pkgs.push(package.as_ref().to_string()),
				None => params.push_str(&format!(
					"&arg[]={}",
					package.as_ref()
				)),
			};
		}

		if !params.is_empty() {
			let url = format!(
				"{}&type=info{}",
				self.prepare_rpc_url(),
				params
			);

			let res = self.request(&url)?;

			pkgs.reserve(res.count);
			for package in res.results {
				let name = package.name.clone();
				self.cache.add(name.clone(), package);

				pkgs.push(name);
			}
		}

		Ok(pkgs
			.drain(..)
			.map(|name| self.cache.get(&name).unwrap().clone())
			.collect())
	}
}

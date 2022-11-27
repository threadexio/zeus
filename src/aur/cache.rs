use std::borrow::Borrow;
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct CacheItem<T> {
	timestamp: Instant,
	ttl: Duration,
	item: T,
}

impl<T> CacheItem<T> {
	pub fn new(item: T, ttl: Duration) -> Self {
		Self { item, ttl, timestamp: Instant::now() }
	}

	pub fn expired(&self) -> bool {
		Instant::now() > self.timestamp + self.ttl
	}

	pub fn get(&self) -> Option<&T> {
		if self.expired() {
			None
		} else {
			Some(&self.item)
		}
	}

	pub fn remove(self) -> Option<T> {
		if self.expired() {
			None
		} else {
			Some(self.item)
		}
	}
}

/// A simple cache for queried packages so we don't
/// create unnecessary traffic.
#[derive(Debug, Clone)]
pub struct Cache<K, V>
where
	K: std::cmp::Eq + std::hash::Hash,
{
	ttl: Duration,
	inner: HashMap<K, CacheItem<V>>,
}

impl<K, V> Cache<K, V>
where
	K: std::cmp::Eq + std::hash::Hash,
{
	pub fn new(ttl: Duration) -> Self {
		Self { ttl, inner: HashMap::new() }
	}

	pub fn set_ttl(&mut self, ttl: Duration) {
		self.ttl = ttl;
	}

	/// Add a new entry in the cache and get the previous value if it existed.
	pub fn add(&mut self, key: K, item: V) -> Option<V> {
		self.inner
			.insert(key, CacheItem::new(item, self.ttl))
			.and_then(|x| x.remove())
	}

	/// Remove the package from the cache.
	pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
	where
		Q: Eq + Hash + ?Sized,
		K: Borrow<Q>,
	{
		self.inner.remove(key).and_then(|x| x.remove())
	}

	/// Get a reference to the stored package.
	pub fn get<Q>(&self, key: &Q) -> Option<&V>
	where
		Q: Eq + Hash + ?Sized,
		K: Borrow<Q>,
	{
		self.inner.get(key).and_then(|x| x.get())
	}
}

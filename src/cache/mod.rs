//! Module providing abstractions to represent caches.

/// The outcome of an eviction from a cache.
///
/// Evictions occur in cache implementations on insert operations in a maxed out cache. This happens
/// to make space for the just inserted key/value pair.
#[derive(Debug, PartialEq, Eq)]
pub enum Eviction<K, V> {
    /// Block eviction with evicted key/value pair on a key/value insertion with an unique key.
    Block { key: K, value: V },

    /// Value eviction on insertion with a key already existing in the cache.
    Value(V),

    /// No eviction when the cache is not maxed out.
    None,
}

/// The outcome of a lookup query from a [`Cache`].
#[derive(Debug, PartialEq, Eq)]
pub enum Lookup<V> {
    /// Cache hit.
    Hit(V),

    /// Cache miss.
    Miss,
}

/// A size bounded map, where certain existing entries are evicted to make space for new entries.
///
/// Implementations follow a well defined criteria to decide which cache blocks to evict in which
/// order. (e.g an LRU cache implementation would evict the least recently used cache blocks).
pub trait Cache<K, V> {
    /// Associated error type.
    type Error;

    /// Inserts the given key/value pair into this cache.
    fn insert(&mut self, key: K, value: V) -> Result<Eviction<K, V>, Self::Error>;

    /// Removes the key/value pair associated with the given key from this cache.
    fn remove(&mut self, key: &K) -> Result<Lookup<V>, Self::Error>;

    /// Removes `(self.len() - new_capacity)` cache blocks to fit the new capacity. If the
    /// difference is non-positive no cache blocks are removed.
    fn shrink(&mut self, new_capacity: usize) -> Result<(), Self::Error>;

    /// Reserves additional memory to accomodate the given number of additional cache blocks.
    fn reserve(&mut self, additional: usize) -> Result<(), Self::Error>;

    /// Queries this cache to find the value associated with given key.
    fn query(&mut self, key: &K) -> Result<Lookup<&V>, Self::Error>;

    /// Returns the current capacity of this cache.
    fn capacity(&self) -> usize;

    /// Returns the number of key/value pairs stored in this cache.
    fn len(&self) -> usize;

    /// Returns whether this cache is maxed out.
    ///
    /// This method simply checks whether the current length of this cache equals its capacity.
    fn is_maxed(&self) -> bool {
        self.len() == self.capacity()
    }

    /// Returns whether this cache is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Remove all items from this cache until it's empty.
    fn clear(&mut self) -> Result<(), Self::Error>;
}

pub mod lru_cache;

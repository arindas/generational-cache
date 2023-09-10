//! Module providing abstractions to represent caches.

/// An evicted value from cache.
#[derive(Debug, PartialEq, Eq)]
pub enum Eviction<K, V> {
    Block { key: K, value: V },
    Value(V),
    None,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Lookup<V> {
    Hit(V),
    Miss,
}

/// A size bounded map, where certain existing entries are evicted to make space for new entires.
pub trait Cache<K, V> {
    type Error;

    fn insert(&mut self, key: K, value: V) -> Result<Eviction<K, V>, Self::Error>;

    fn remove(&mut self, key: &K) -> Result<Lookup<V>, Self::Error>;

    fn shrink(&mut self, new_capacity: usize) -> Result<(), Self::Error>;

    fn reserve(&mut self, additional: usize) -> Result<(), Self::Error>;

    fn query(&mut self, key: &K) -> Result<Lookup<&V>, Self::Error>;

    fn capacity(&self) -> usize;

    fn len(&self) -> usize;

    fn is_maxed(&self) -> bool {
        self.len() == self.capacity()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn clear(&mut self) -> Result<(), Self::Error>;
}

pub mod lru_cache;

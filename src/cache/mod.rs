pub trait Cache<K, V> {
    type Error;

    fn insert(&mut self, key: K, value: V) -> Result<(), Self::Error>;

    fn remove(&mut self, key: &K) -> Result<V, Self::Error>;

    fn query(&mut self, key: &K) -> Result<&V, Self::Error>;

    fn capacity(&self) -> usize;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn clear(&mut self);
}

pub mod lru_cache;

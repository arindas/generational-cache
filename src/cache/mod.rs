pub enum Evict<K, V> {
    Block { key: K, value: V },
    Value(V),
    None,
}

pub trait Cache<K, V> {
    type Error;

    fn insert(&mut self, key: K, value: V) -> Result<Evict<K, V>, Self::Error>;

    fn remove(&mut self, key: &K) -> Result<V, Self::Error>;

    fn query(&mut self, key: &K) -> Result<&V, Self::Error>;

    fn capacity(&self) -> usize;

    fn len(&self) -> usize;

    fn is_maxed(&self) -> bool {
        self.len() == self.capacity()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn clear(&mut self);
}

pub mod lru_cache;

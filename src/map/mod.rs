pub trait Map<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V>;

    fn get(&self, key: &K) -> Option<&V>;

    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    fn remove(&mut self, key: &K) -> Option<V>;

    fn clear(&mut self);
}

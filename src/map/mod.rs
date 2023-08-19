pub trait Map<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V>;

    fn get(&self, key: &K) -> Option<&V>;

    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    fn remove(&mut self, key: &K) -> Option<V>;

    fn clear(&mut self);

    fn is_empty(&self) -> bool;
}

pub mod impls;

#[doc(hidden)]
pub mod tests {
    use super::Map;

    pub fn _test_map_consistency<M: Map<usize, usize>>(mut map: M) {
        map.clear();

        assert!(map.is_empty());

        const NUM_ENTRIES: usize = 10;

        for i in 0..NUM_ENTRIES {
            map.insert(i, i);
        }

        for i in 0..NUM_ENTRIES {
            assert_eq!(map.get(&i), Some(&i));
        }

        for i in 0..NUM_ENTRIES {
            let val = map.get_mut(&i);
            val.map(|x| *x = i + 1);
        }

        for i in 0..NUM_ENTRIES {
            assert_eq!(map.get(&i), Some(&(i + 1)));
        }

        assert_eq!(map.insert(0, NUM_ENTRIES), Some(1));
        assert_eq!(map.get(&0), Some(&NUM_ENTRIES));

        assert_eq!(map.remove(&0), Some(NUM_ENTRIES));

        assert!(map.get(&0).is_none());

        map.clear();
        assert!(map.is_empty());
    }
}

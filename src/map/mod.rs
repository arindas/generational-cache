//! Module providing abstractions to implement maps.

/// An abstrct mapping from a set of keys to a set of values.
pub trait Map<K, V> {
    type Error: core::fmt::Debug;

    fn insert(&mut self, key: K, value: V) -> Result<Option<V>, Self::Error>;

    fn get(&self, key: &K) -> Option<&V>;

    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    fn remove(&mut self, key: &K) -> Option<V>;

    fn clear(&mut self) -> Result<(), Self::Error>;

    fn is_empty(&self) -> bool;

    fn capacity(&self) -> Option<usize>;

    fn len(&self) -> usize;
}

pub mod impls;

#[doc(hidden)]
pub mod tests {
    use super::Map;

    pub fn _test_map_consistency<M: Map<usize, usize> + Default>() {
        let mut map = M::default();

        map.clear().unwrap();

        assert!(map.is_empty());

        let num_entries: usize = map.capacity().unwrap_or(10);

        for i in 0..num_entries {
            assert!(map.insert(i, i).unwrap().is_none());
        }

        for i in 0..num_entries {
            assert_eq!(map.get(&i), Some(&i));
        }

        for i in 0..num_entries {
            let val = map.get_mut(&i);

            if let Some(val) = val {
                *val += 1;
            }
        }

        for i in 0..num_entries {
            assert_eq!(map.get(&i), Some(&(i + 1)));
        }

        assert_eq!(map.insert(0, num_entries).unwrap(), Some(1));
        assert_eq!(map.get(&0), Some(&num_entries));

        assert_eq!(map.len(), num_entries);

        if let Some(capacity) = map.capacity() {
            assert_eq!(capacity, map.len());
        }

        if let (Some(_), Ok(_)) = (map.capacity(), map.insert(num_entries, num_entries)) {
            unreachable!("No error on capacity breach.")
        }

        assert_eq!(map.remove(&0), Some(num_entries));

        assert!(map.get(&0).is_none());

        map.clear().unwrap();
        assert!(map.is_empty());
    }
}

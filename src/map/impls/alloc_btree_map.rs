//! Module providing a map implementation based on [alloc::collections::BTreeMap].

extern crate alloc;

use crate::map::Map;
use alloc::collections::BTreeMap;

/// A [Map] implementation based on [alloc::collections::BTreeMap].
pub struct AllocBTreeMap<K, V> {
    btree_map: BTreeMap<K, V>,
}

impl<K, V> Default for AllocBTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> AllocBTreeMap<K, V> {
    pub fn with_btree_map(btree_map: BTreeMap<K, V>) -> Self {
        Self { btree_map }
    }

    pub fn new() -> Self {
        Self {
            btree_map: BTreeMap::new(),
        }
    }
}

impl<K: Ord, V> Map<K, V> for AllocBTreeMap<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.btree_map.insert(key, value)
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.btree_map.get(key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.btree_map.get_mut(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.btree_map.remove(key)
    }

    fn clear(&mut self) {
        self.btree_map.clear()
    }

    fn is_empty(&self) -> bool {
        self.btree_map.is_empty()
    }
}

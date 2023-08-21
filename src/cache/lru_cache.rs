use crate::{
    cache::{Cache, Eviction},
    collections::list::{Link, LinkedList, LinkedListArenaEntry, ListError},
    map::Map,
    vector::Vector,
};
use core::mem;

extern crate alloc;

#[derive(Clone, Copy)]
pub struct Block<K, T> {
    pub key: K,
    pub value: T,
}

pub type CacheBlockArenaEntry<K, T> = LinkedListArenaEntry<Block<K, T>>;

pub type BlockList<V, K, T> = LinkedList<V, Block<K, T>>;

pub struct LRUCache<V, K, T, M> {
    block_list: BlockList<V, K, T>,
    block_refs: M,
}

impl<V, K, T, M> LRUCache<V, K, T, M>
where
    V: Vector<CacheBlockArenaEntry<K, T>>,
    M: Map<K, Link>,
{
    fn with_block_list_and_block_refs(block_list: BlockList<V, K, T>, block_refs: M) -> Self {
        Self {
            block_list,
            block_refs,
        }
    }

    pub fn least_recent(&self) -> Option<(&K, &T)> {
        let block = self.block_list.peek_front()?;
        Some((&block.key, &block.value))
    }

    pub fn most_recent(&self) -> Option<(&K, &T)> {
        let block = self.block_list.peek_back()?;
        Some((&block.key, &block.value))
    }
}

impl<V, K, T, M> LRUCache<V, K, T, M>
where
    V: Vector<CacheBlockArenaEntry<K, T>>,
    M: Map<K, Link>,
{
    pub fn with_backing_vector_and_map(vector: V, map: M) -> Self {
        let block_list = LinkedList::with_backing_vector(vector);
        Self::with_block_list_and_block_refs(block_list, map)
    }
}

impl<V, K, T, M> LRUCache<V, K, T, M>
where
    V: Vector<CacheBlockArenaEntry<K, T>>,
    M: Map<K, Link> + Default,
{
    pub fn with_backing_vector(vector: V) -> Self {
        Self::with_backing_vector_and_map(vector, M::default())
    }
}

impl<V, K, T, M> Default for LRUCache<V, K, T, M>
where
    V: Vector<CacheBlockArenaEntry<K, T>> + Default,
    M: Map<K, Link> + Default,
{
    fn default() -> Self {
        Self::with_backing_vector(V::default())
    }
}

#[derive(PartialEq, Debug)]
pub enum CacheError {
    ListError(ListError),
    ListUnderflow,
    MapListInconsistent,
    CacheMiss,
}

#[allow(unused)]
impl<V, K, T, M> Cache<K, T> for LRUCache<V, K, T, M>
where
    V: Vector<CacheBlockArenaEntry<K, T>>,
    M: Map<K, Link>,
    K: Copy,
{
    type Error = CacheError;

    fn insert(&mut self, key: K, value: T) -> Result<Eviction<K, T>, Self::Error> {
        if let Some(link) = self.block_refs.get(&key) {
            self.block_list
                .shift_push_back(link)
                .ok_or(CacheError::MapListInconsistent)?;

            let block = self
                .block_list
                .get_mut(link)
                .ok_or(CacheError::MapListInconsistent)?;

            return Ok(Eviction::Value(mem::replace(&mut block.value, value)));
        }

        let eviction = if self.is_maxed() {
            let Block { key, value } = self
                .block_list
                .pop_front()
                .ok_or(CacheError::ListUnderflow)?;

            self.block_refs.remove(&key);

            Eviction::Block { key, value }
        } else {
            Eviction::None
        };

        let link = self
            .block_list
            .push_back(Block { key, value })
            .map_err(CacheError::ListError)?;

        self.block_refs.insert(key, link);

        Ok(eviction)
    }

    fn remove(&mut self, key: &K) -> Result<T, Self::Error> {
        let link = self.block_refs.remove(key).ok_or(CacheError::CacheMiss)?;

        let block = self
            .block_list
            .remove(&link)
            .ok_or(CacheError::MapListInconsistent)?;

        Ok(block.value)
    }

    fn query(&mut self, key: &K) -> Result<&T, Self::Error> {
        let link = self.block_refs.get(key).ok_or(CacheError::CacheMiss)?;

        self.block_list
            .shift_push_back(link)
            .ok_or(CacheError::MapListInconsistent)?;

        let block = self
            .block_list
            .get(link)
            .ok_or(CacheError::MapListInconsistent)?;

        Ok(&block.value)
    }

    fn capacity(&self) -> usize {
        self.block_list.capacity()
    }

    fn len(&self) -> usize {
        self.block_list.len()
    }

    fn is_empty(&self) -> bool {
        self.block_list.is_empty()
    }

    fn clear(&mut self) {
        self.block_list.clear();
        self.block_refs.clear();
    }
}

#[doc(hidden)]
pub mod tests {

    use super::{Cache, CacheBlockArenaEntry, CacheError, Eviction, LRUCache, Link, Map, Vector};

    pub fn _test_cache_correctness<VX, VY, M>(zero_capacity_vec: VX, test_vec: VY)
    where
        VX: Vector<CacheBlockArenaEntry<usize, usize>>,
        VY: Vector<CacheBlockArenaEntry<usize, usize>>,
        M: Map<usize, Link> + Default,
    {
        assert_eq!(
            zero_capacity_vec.capacity(),
            0,
            "Zero capacity vector provider yielded vector of non zero capacity."
        );

        let mut cache = LRUCache::<_, _, _, M>::with_backing_vector(zero_capacity_vec);

        match cache.insert(0, 0) {
            Err(CacheError::ListUnderflow) => {}
            _ => unreachable!("Wrong error on list underflow."),
        };

        let mut cache = LRUCache::<_, _, _, M>::with_backing_vector(test_vec);

        let capacity = cache.capacity();

        assert!(
            capacity > 3,
            "Too small capacity: {} to run meaningful tests.",
            capacity
        );

        for i in 0..cache.capacity() {
            assert_eq!(cache.insert(i, i).unwrap(), Eviction::None);
        }

        assert_eq!(cache.least_recent().unwrap(), (&0, &0));

        assert_eq!(
            cache.insert(capacity, capacity).unwrap(),
            Eviction::Block { key: 0, value: 0 }
        );

        assert_eq!(cache.query(&1).unwrap(), &1);

        assert_eq!(cache.least_recent().unwrap(), (&2, &2));
        assert_eq!(cache.most_recent().unwrap(), (&1, &1));

        assert_eq!(
            cache.insert(capacity + 1, capacity + 1).unwrap(),
            Eviction::Block { key: 2, value: 2 }
        );

        assert_eq!(
            cache.insert(capacity, capacity + 2).unwrap(),
            Eviction::Value(capacity)
        );

        assert_eq!(cache.most_recent().unwrap(), (&capacity, &(capacity + 2)));
    }
}

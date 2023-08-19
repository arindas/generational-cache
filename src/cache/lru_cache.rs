use super::{Cache, Evict};
use crate::{
    arena::Entry,
    collections::list::{Link, LinkedList, ListError, Node},
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

pub type BlockList<V, K, T> = LinkedList<V, Block<K, T>>;

pub struct LRUCache<V, K, T, M> {
    block_list: BlockList<V, K, T>,
    block_refs: M,
}

impl<V, K, T, M> LRUCache<V, K, T, M>
where
    V: Vector<Entry<Node<Block<K, T>>>>,
    M: Map<K, Link>,
{
    pub fn with_block_list_and_block_refs(
        mut block_list: BlockList<V, K, T>,
        mut block_refs: M,
    ) -> Self {
        block_list.clear();
        block_refs.clear();

        Self {
            block_list,
            block_refs,
        }
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
    V: Vector<Entry<Node<Block<K, T>>>>,
    M: Map<K, Link>,
    K: Copy,
{
    type Error = CacheError;

    fn insert(&mut self, key: K, value: T) -> Result<Evict<K, T>, Self::Error> {
        if let Some(link) = self.block_refs.get(&key) {
            self.block_list
                .shift_push_back(link)
                .ok_or(CacheError::MapListInconsistent)?;

            let block = self
                .block_list
                .get_mut(link)
                .ok_or(CacheError::MapListInconsistent)?;

            return Ok(Evict::Value(mem::replace(&mut block.value, value)));
        }

        let evict = if self.is_maxed() {
            let Block { key, value } = self
                .block_list
                .pop_front()
                .ok_or(CacheError::ListUnderflow)?;

            self.block_refs.remove(&key);

            Evict::Block { key, value }
        } else {
            Evict::None
        };

        let link = self
            .block_list
            .push_back(Block { key, value })
            .map_err(CacheError::ListError)?;

        self.block_refs.insert(key, link);

        Ok(evict)
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

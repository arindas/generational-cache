use super::Cache;
use crate::{
    arena::Entry,
    collections::list::{Link, LinkedList, ListError, Node},
    map::Map,
    vector::Vector,
};

extern crate alloc;

#[allow(unused)]
pub struct Block<K, T> {
    pub key: K,
    pub value: T,
}

type BlockList<V, K, T> = LinkedList<V, Block<K, T>>;

#[allow(unused)]
pub struct LRUCache<V, K, T, M> {
    block_list: BlockList<V, K, T>,
    block_refs: M,
}

impl<V, K, T, M> LRUCache<V, K, T, M>
where
    V: Vector<Entry<Node<Block<K, T>>>>,
    M: Map<K, Link>,
{
    pub fn with_block_list_and_refs(mut block_list: BlockList<V, K, T>, mut block_refs: M) -> Self {
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
    CacheMiss,
}

#[allow(unused)]
impl<V, K, T, M> Cache<K, T> for LRUCache<V, K, T, M>
where
    V: Vector<Entry<Node<Block<K, T>>>>,
    M: Map<K, Link>,
{
    type Error = CacheError;

    fn insert(&mut self, key: K, value: T) -> Result<(), Self::Error> {
        todo!()
    }

    fn remove(&mut self, key: &K) -> Result<T, Self::Error> {
        todo!()
    }

    fn query(&mut self, key: &K) -> Result<&T, Self::Error> {
        todo!()
    }

    fn capacity(&self) -> usize {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }
}

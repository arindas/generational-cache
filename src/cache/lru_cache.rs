use super::Cache;
use crate::{
    arena::Entry,
    collections::list::{LinkedList, ListError, Node},
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
pub struct LRUCache<V, K, T> {
    block_list: BlockList<V, K, T>,
}

impl<V, K, T> LRUCache<V, K, T>
where
    V: Vector<Entry<Node<Block<K, T>>>>,
{
    pub fn with_backing_block_list(mut block_list: BlockList<V, K, T>) -> Self {
        block_list.clear();

        Self { block_list }
    }
}

#[derive(PartialEq, Debug)]
pub enum CacheError {
    ListError(ListError),
    CacheMiss,
}

#[allow(unused)]
impl<V, K, T> Cache<K, T> for LRUCache<V, K, T>
where
    V: Vector<Entry<Node<Block<K, T>>>>,
    K: PartialEq + Eq,
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
}

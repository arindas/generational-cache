extern crate alloc;

use crate::{
    arena::{Arena, Entry},
    cache::lru_cache::{Block, LRUCache},
    collections::list::{Link, LinkedList, Node},
    map::impls::allocbtreemap::AllocBTreeMap,
    vector::Vector,
};
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};

pub struct AllocVec<T> {
    vec: Vec<T>,
}

impl<T> Default for AllocVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AllocVec<T> {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
        }
    }
}

impl<T> DerefMut for AllocVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec[..]
    }
}

impl<T> Deref for AllocVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.vec[..]
    }
}

impl<T> Vector<T> for AllocVec<T> {
    fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    fn push(&mut self, item: T) {
        self.vec.push(item)
    }

    fn clear(&mut self) {
        self.vec.clear()
    }
}

pub type AllocLinkedList<T> = LinkedList<AllocLinkedListArenaVec<T>, T>;
pub type AllocLinkedListArenaVec<T> = AllocVec<Entry<Node<T>>>;

pub type AllocLinkedListAllocBTreeLRUCache<K, T> =
    LRUCache<AllocLinkedListArenaVec<Block<K, T>>, K, T, AllocBTreeMap<K, Link>>;

pub struct AllocLinkedListBTreeLRUCache<K, T> {
    cache: AllocLinkedListAllocBTreeLRUCache<K, T>,
}

impl<K: Ord, T> AllocLinkedListBTreeLRUCache<K, T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            cache: AllocLinkedListAllocBTreeLRUCache::with_block_list_and_block_refs(
                AllocLinkedList::with_backing_arena(Arena::with_vector(
                    AllocLinkedListArenaVec::with_capacity(capacity),
                )),
                AllocBTreeMap::new(),
            ),
        }
    }
}

impl<K, T> DerefMut for AllocLinkedListBTreeLRUCache<K, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cache
    }
}

impl<K, T> Deref for AllocLinkedListBTreeLRUCache<K, T> {
    type Target = AllocLinkedListAllocBTreeLRUCache<K, T>;

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}

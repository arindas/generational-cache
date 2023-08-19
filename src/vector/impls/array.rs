use crate::{
    arena::{Arena, Entry},
    cache::lru_cache::{Block, LRUCache},
    collections::list::{Link, LinkedList, Node},
    map::impls::alloc_btree_map::AllocBTreeMap,
    vector::Vector,
};
use core::{
    marker::Copy,
    ops::{Deref, DerefMut},
};

pub struct Array<T, const N: usize> {
    buffer: [T; N],
    len: usize,
}

impl<T, const N: usize> Array<T, N> {
    pub fn with_buffer(buffer: [T; N]) -> Self {
        Self { buffer, len: 0 }
    }
}

impl<T, const N: usize> Array<T, N>
where
    T: Copy + Default,
{
    pub fn new() -> Self {
        Self::with_buffer([Default::default(); N])
    }
}

impl<T, const N: usize> Default for Array<T, N>
where
    T: Copy + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> DerefMut for Array<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer[0..self.len]
    }
}

impl<T, const N: usize> Deref for Array<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.buffer[0..self.len]
    }
}

impl<T, const N: usize> Vector<T> for Array<T, N> {
    fn capacity(&self) -> usize {
        N
    }

    fn push(&mut self, item: T) {
        self.buffer[self.len] = item;
        self.len += 1;
    }

    fn clear(&mut self) {
        self.len = 0;
    }
}

pub type ArrayLinkedList<T, const N: usize> = LinkedList<ArrayLinkedListArenaVec<T, N>, T>;
pub type ArrayLinkedListArenaVec<T, const N: usize> = Array<Entry<Node<T>>, N>;

pub type ArrayLinkedListAllocBTreeLRUCacheRaw<K, T, const N: usize> =
    LRUCache<ArrayLinkedListArenaVec<Block<K, T>, N>, K, T, AllocBTreeMap<K, Link>>;

pub struct ArrayLinkedListAllocBTreeLRUCache<K, T, const N: usize> {
    cache: ArrayLinkedListAllocBTreeLRUCacheRaw<K, T, N>,
}

impl<K: Ord + Copy, T: Copy, const N: usize> ArrayLinkedListAllocBTreeLRUCache<K, T, N> {
    pub fn new() -> Self {
        Self {
            cache: ArrayLinkedListAllocBTreeLRUCacheRaw::with_block_list_and_block_refs(
                ArrayLinkedList::with_backing_arena(Arena::with_vector(
                    ArrayLinkedListArenaVec::new(),
                )),
                AllocBTreeMap::new(),
            ),
        }
    }
}

impl<K, T, const N: usize> DerefMut for ArrayLinkedListAllocBTreeLRUCache<K, T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cache
    }
}

impl<K, T, const N: usize> Deref for ArrayLinkedListAllocBTreeLRUCache<K, T, N> {
    type Target = ArrayLinkedListAllocBTreeLRUCacheRaw<K, T, N>;

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}

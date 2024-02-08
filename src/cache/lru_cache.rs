//! Module providing abstractions to represent an LRUCache.
//!
//! ## Usage
//!
//! ```rust
//! use generational_cache::prelude::*;
//!
//! const CAPACITY: usize = 3;
//!
//! let mut cache = LRUCache::<_, i32, u64, AllocBTreeMap<_, _>>::with_backing_vector(Array::<_, CAPACITY>::new());
//!
//! cache.insert(-1, 1).unwrap();
//! cache.insert(-2, 2).unwrap();
//! cache.insert(-3, 3).unwrap();
//!
//! assert_eq!(cache.least_recent().unwrap(), (&-1, &1));
//! assert_eq!(cache.most_recent().unwrap(), (&-3, &3));
//!
//! assert_eq!(cache.insert(-4, 4).unwrap(), Eviction::Block { key: -1, value: 1});
//!
//! assert_eq!(cache.least_recent().unwrap(), (&-2, &2));
//! assert_eq!(cache.most_recent().unwrap(), (&-4, &4));
//!
//! assert_eq!(cache.insert(-2, 42).unwrap(), Eviction::Value(2));
//!
//! assert_eq!(cache.least_recent().unwrap(), (&-3, &3));
//! assert_eq!(cache.most_recent().unwrap(), (&-2, &42));
//!
//! assert_eq!(cache.remove(&-42).unwrap(), Lookup::Miss);
//! assert_eq!(cache.query(&-42).unwrap(), Lookup::Miss);
//!
//! assert_eq!(cache.query(&-3).unwrap(), Lookup::Hit(&3));
//!
//! assert_eq!(cache.least_recent().unwrap(), (&-4, &4));
//! assert_eq!(cache.most_recent().unwrap(), (&-3, &3));
//!
//! assert_eq!(cache.remove(&-2).unwrap(), Lookup::Hit(42));
//!
//! assert_eq!(cache.query(&-2).unwrap(), Lookup::Miss);
//!
//! // zero capacity LRUCache is unusable
//! let mut cache = LRUCache::<_, i32, u64, AllocBTreeMap<_, _>>::with_backing_vector(Array::<_, 0_usize>::new());
//!
//! match cache.insert(0, 0) {
//!     Err(LRUCacheError::ListUnderflow) => {}
//!     _ => unreachable!("Wrong error on list underflow."),
//! };
//!
//! ```

use crate::{
    cache::{Cache, Eviction},
    collections::list::{Link, LinkedList, LinkedListArenaEntry, ListError},
    map::Map,
    vector::Vector,
};
use core::{
    fmt::{Debug, Display},
    mem,
};

use super::Lookup;

extern crate alloc;

/// A cache block containing a key value pair.
#[derive(Clone, Copy)]
pub struct Block<K, T> {
    pub key: K,
    pub value: T,
}

/// Alias representing block entries for storage in a generational arena.
pub type LRUCacheBlockArenaEntry<K, T> = LinkedListArenaEntry<Block<K, T>>;

/// A generational [`Arena`](crate::arena::Arena) backed LRU cache implementation.
///
/// This [`Cache`] implementation always evicts the least-recently-used (LRU) key/value pair. It
/// uses a [`LinkedList`] for storing the underlying cache block entries to maintain the order
/// in which they were inserted into the cache.
///
/// It uses a generational [`Arena`](crate::arena::Arena) for allocating the underlying
/// [`LinkedList`] which stores the cache blocks. It uses a [`Map`] for maintaining the mapping
/// from keys to the nodes storing the respective cache blocks in the [`LinkedList`].
///
/// ### Type parameters
/// - `V: Vector<LRUCacheBlockArenaEntry<K, T>>`
///     Used as the backing vector for the underlying [`Arena`](crate::arena::Arena).
/// - `K`
///     The Key type.
/// - `V`
///     The Value type.
/// - `M: Map<K, Link>`
///     Used to store a mapping from the keys to links in the linked list.
///
pub struct LRUCache<V, K, T, M> {
    block_list: LinkedList<V, Block<K, T>>,
    block_refs: M,

    capacity: usize,
}

impl<V, K, T, M> LRUCache<V, K, T, M>
where
    V: Vector<LRUCacheBlockArenaEntry<K, T>>,
    M: Map<K, Link>,
{
    /// Returns the least recently used key/value pair.
    pub fn least_recent(&self) -> Option<(&K, &T)> {
        let block = self.block_list.peek_front()?;
        Some((&block.key, &block.value))
    }

    /// Returns the most recently used key/value pair.
    pub fn most_recent(&self) -> Option<(&K, &T)> {
        let block = self.block_list.peek_back()?;
        Some((&block.key, &block.value))
    }
}

impl<V, K, T, M> LRUCache<V, K, T, M>
where
    V: Vector<LRUCacheBlockArenaEntry<K, T>>,
    M: Map<K, Link>,
{
    /// Creates an [`LRUCache`] instance with the given the backing [`Vector`] and [`Map`]
    /// implementation instances.
    pub fn with_backing_vector_and_map(vector: V, map: M) -> Self {
        let block_list = LinkedList::with_backing_vector(vector);
        let capacity = block_list.capacity();

        Self {
            block_list,
            block_refs: map,
            capacity,
        }
    }
}

impl<V, K, T, M> LRUCache<V, K, T, M>
where
    V: Vector<LRUCacheBlockArenaEntry<K, T>>,
    M: Map<K, Link> + Default,
{
    /// Creates an [`LRUCache`] instance with the given [`Vector`] implementation instance
    /// and the default [`Map`] implementation value.
    pub fn with_backing_vector(vector: V) -> Self {
        Self::with_backing_vector_and_map(vector, M::default())
    }
}

impl<V, K, T, M> Default for LRUCache<V, K, T, M>
where
    V: Vector<LRUCacheBlockArenaEntry<K, T>> + Default,
    M: Map<K, Link> + Default,
{
    fn default() -> Self {
        Self::with_backing_vector(V::default())
    }
}

/// Error type associated with [`LRUCache`] operations.
#[derive(Debug)]
pub enum LRUCacheError<VE, ME> {
    /// Used when there is an error on an operation in the underlying list.
    ListError(ListError<VE>),

    /// Used when attempting to remove elements from the underlying list when its empty.
    ListUnderflow,

    /// Used when the underlying map and list instances contain an inconsistent view
    /// of the entries allocated in the LRUCache
    MapListInconsistent,

    /// Used when there is an error on an operation in the underlying map..
    MapError(ME),
}

impl<VE, ME> Display for LRUCacheError<VE, ME>
where
    VE: Debug,
    ME: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[allow(unused)]
impl<V, K, T, M> Cache<K, T> for LRUCache<V, K, T, M>
where
    V: Vector<LRUCacheBlockArenaEntry<K, T>>,
    M: Map<K, Link>,
    K: Copy,
{
    type Error = LRUCacheError<V::Error, M::Error>;

    fn insert(&mut self, key: K, value: T) -> Result<Eviction<K, T>, Self::Error> {
        if let Some(link) = self.block_refs.get(&key) {
            self.block_list
                .shift_push_back(link)
                .ok_or(Self::Error::MapListInconsistent)?;

            let block = self
                .block_list
                .get_mut(link)
                .ok_or(Self::Error::MapListInconsistent)?;

            return Ok(Eviction::Value(mem::replace(&mut block.value, value)));
        }

        let eviction = if self.is_maxed() {
            let Block { key, value } = self
                .block_list
                .pop_front()
                .ok_or(Self::Error::ListUnderflow)?;

            self.block_refs.remove(&key);

            Eviction::Block { key, value }
        } else {
            Eviction::None
        };

        let link = self
            .block_list
            .push_back(Block { key, value })
            .map_err(Self::Error::ListError)?;

        self.block_refs
            .insert(key, link)
            .map_err(Self::Error::MapError)?;

        Ok(eviction)
    }

    fn remove(&mut self, key: &K) -> Result<Lookup<T>, Self::Error> {
        match self.block_refs.remove(key) {
            Some(link) => self
                .block_list
                .remove(&link)
                .map(|x| Lookup::Hit(x.value))
                .ok_or(Self::Error::MapListInconsistent),
            _ => Ok(Lookup::Miss),
        }
    }

    fn shrink(&mut self, new_capacity: usize) -> Result<(), Self::Error> {
        if new_capacity >= self.capacity() {
            return Ok(());
        }

        while self.len() > new_capacity {
            let Block { key, value } = self
                .block_list
                .pop_front()
                .ok_or(Self::Error::ListUnderflow)?;

            self.block_refs.remove(&key);
        }

        self.capacity = new_capacity;

        Ok(())
    }

    fn reserve(&mut self, additional: usize) -> Result<(), Self::Error> {
        self.block_list
            .reserve(additional)
            .map_err(Self::Error::ListError)?;

        self.capacity += additional;

        Ok(())
    }

    fn query(&mut self, key: &K) -> Result<Lookup<&T>, Self::Error> {
        match self.block_refs.get(key) {
            Some(link) => {
                self.block_list
                    .shift_push_back(link)
                    .ok_or(Self::Error::MapListInconsistent)?;

                self.block_list
                    .get(link)
                    .map(|x| Lookup::Hit(&x.value))
                    .ok_or(Self::Error::MapListInconsistent)
            }
            _ => Ok(Lookup::Miss),
        }
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn len(&self) -> usize {
        self.block_list.len()
    }

    fn is_empty(&self) -> bool {
        self.block_list.is_empty()
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        self.block_list.clear().map_err(Self::Error::ListError)?;
        self.block_refs.clear().map_err(Self::Error::MapError)?;

        Ok(())
    }
}

#[doc(hidden)]
pub mod tests {

    use super::{
        Cache, Eviction, LRUCache, LRUCacheBlockArenaEntry, LRUCacheError, Link, Lookup, Map,
        Vector,
    };

    pub fn _test_cache_correctness<VX, VY, M>(zero_capacity_vec: VX, test_vec: VY)
    where
        VX: Vector<LRUCacheBlockArenaEntry<usize, usize>>,
        VY: Vector<LRUCacheBlockArenaEntry<usize, usize>>,
        M: Map<usize, Link> + Default,
    {
        assert_eq!(
            zero_capacity_vec.capacity(),
            0,
            "Zero capacity vector provider yielded vector of non zero capacity."
        );

        let mut cache = LRUCache::<_, _, _, M>::with_backing_vector(zero_capacity_vec);

        assert!(cache.is_empty());

        match cache.insert(0, 0) {
            Err(LRUCacheError::ListUnderflow) => {}
            _ => unreachable!("Wrong error on list underflow."),
        };

        let mut cache = LRUCache::<_, _, _, M>::with_backing_vector(test_vec);

        let capacity = cache.capacity();

        assert!(
            capacity > 3,
            "Too small capacity: {} to run meaningful tests.",
            capacity
        );

        assert!(cache.is_empty());

        for i in 0..cache.capacity() {
            assert_eq!(cache.insert(i, i).unwrap(), Eviction::None);
        }

        assert_eq!(cache.least_recent().unwrap(), (&0, &0));

        assert_eq!(
            cache.insert(capacity, capacity).unwrap(),
            Eviction::Block { key: 0, value: 0 }
        );

        assert_eq!(cache.query(&1).unwrap(), Lookup::Hit(&1));

        assert_eq!(cache.least_recent().unwrap(), (&2, &2));
        assert_eq!(cache.most_recent().unwrap(), (&1, &1));

        assert_eq!(cache.remove(&(capacity + 1)).unwrap(), Lookup::Miss);
        assert_eq!(cache.query(&(capacity + 1)).unwrap(), Lookup::Miss);

        assert_eq!(
            cache.insert(capacity + 1, capacity + 1).unwrap(),
            Eviction::Block { key: 2, value: 2 }
        );

        assert_eq!(
            cache.remove(&(capacity + 1)).unwrap(),
            Lookup::Hit(capacity + 1)
        );

        assert_eq!(cache.remove(&(capacity + 1)).unwrap(), Lookup::Miss);
        assert_eq!(cache.query(&(capacity + 1)).unwrap(), Lookup::Miss);

        assert_eq!(
            cache.insert(capacity, capacity + 2).unwrap(),
            Eviction::Value(capacity)
        );

        assert_eq!(cache.most_recent().unwrap(), (&capacity, &(capacity + 2)));

        cache.clear().unwrap();

        assert!(cache.is_empty());

        for i in 0..cache.capacity() {
            assert_eq!(cache.insert(i, i).unwrap(), Eviction::None);
        }

        assert_eq!(cache.least_recent().unwrap(), (&0, &0));

        const ADDITIONAL: usize = 5;

        let result = cache.reserve(ADDITIONAL);

        if result.is_ok() {
            let old_len = cache.len();
            for i in 0..ADDITIONAL {
                assert_eq!(cache.insert(i + old_len, i).unwrap(), Eviction::None);
            }
        }

        let old_capacity = cache.capacity();

        cache.shrink(0).unwrap();

        assert!(cache.is_maxed());

        match cache.insert(0, 0) {
            Err(LRUCacheError::ListUnderflow) => {}
            _ => unreachable!("Wrong error on list underflow."),
        };

        assert!(cache.is_empty());

        cache.reserve(old_capacity).unwrap();
        cache.shrink(old_capacity).unwrap();

        assert_eq!(cache.capacity(), old_capacity);

        for i in 0..cache.capacity() {
            assert_eq!(cache.insert(i, i).unwrap(), Eviction::None);
        }

        cache.clear().unwrap();

        assert!(cache.is_empty());
    }
}

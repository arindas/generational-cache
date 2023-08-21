#![doc = include_str!("../README.md")]
#![no_std]
#![deny(unsafe_code)]

pub mod arena;
pub mod cache;
pub mod collections;
pub mod map;
pub mod vector;

pub mod prelude {
    pub use super::{
        cache::{
            lru_cache::{LRUCache, LRUCacheError},
            Cache, Eviction,
        },
        collections::list::{Link, LinkedList, ListError},
        map::{impls::alloc_btree_map::AllocBTreeMap, Map},
        vector::{
            impls::{alloc_vec::AllocVec, array::Array},
            Vector,
        },
    };
}

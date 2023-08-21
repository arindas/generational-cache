use generational_cache::{
    cache::lru_cache::{self},
    map::{self, impls::alloc_btree_map::AllocBTreeMap},
    vector::impls::{alloc_vec::AllocVec, array::Array},
};

const TEST_CAPACITY: usize = 1 << 4;

#[test]
fn test_alloc_btree_map_consistency() {
    map::tests::_test_map_consistency(AllocBTreeMap::new());
}

#[test]
fn test_alloc_btree_alloc_vec_backed_lru_cache_consistency() {
    lru_cache::tests::_test_cache_correctness::<_, _, AllocBTreeMap<_, _>>(
        AllocVec::with_capacity(0),
        AllocVec::with_capacity(TEST_CAPACITY),
    );
}

#[test]
fn test_alloc_btree_array_vec_backed_lru_cache_consistency() {
    lru_cache::tests::_test_cache_correctness::<_, _, AllocBTreeMap<_, _>>(
        Array::<_, 0>::new(),
        Array::<_, TEST_CAPACITY>::new(),
    );
}

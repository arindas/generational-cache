use generational_cache::map::{self, impls::alloc_btree_map::AllocBTreeMap};

#[test]
fn test_allocbtreemap_consistency() {
    map::tests::_test_map_consistency(AllocBTreeMap::new());
}

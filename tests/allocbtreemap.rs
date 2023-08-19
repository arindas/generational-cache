use generational_cache::map::{self, impls::allocbtreemap::AllocBTreeMap};

#[test]
fn test_allocbtreemap_consistency() {
    map::tests::_test_map_consistency(AllocBTreeMap::new());
}

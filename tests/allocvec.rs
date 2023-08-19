use generational_cache::{
    arena::{self, Arena, Entry},
    collections::list::{self, LinkedList, Node},
    vector::{self, impls::allocvec::AllocVec},
};

const TEST_CAPACITY: usize = 1 << 4;

pub fn allocvec_backed_arena<T>(capacity: usize) -> Arena<AllocVec<Entry<T>>, T> {
    Arena::with_vector(AllocVec::<Entry<T>>::with_capacity(capacity))
}

pub fn allocvec_backed_list<T>(capacity: usize) -> LinkedList<AllocVec<Entry<Node<T>>>, T> {
    LinkedList::with_backing_arena(allocvec_backed_arena(capacity))
}

#[test]
fn test_allocvec_vector_consitency() {
    vector::tests::_test_vector_consistency(AllocVec::<usize>::with_capacity(TEST_CAPACITY));
}

#[test]
fn test_allocvec_arena_free_entries_init() {
    arena::tests::_test_arena_free_entries_init(allocvec_backed_arena::<()>(TEST_CAPACITY));
}

#[test]
fn test_allocvec_arena_insert() {
    arena::tests::_test_arena_insert(allocvec_backed_arena::<i32>(TEST_CAPACITY));
}

#[test]
fn test_allocvec_arena_remove() {
    arena::tests::_test_arena_remove(allocvec_backed_arena::<i32>(TEST_CAPACITY));
}

#[test]
fn test_allocvec_list_invariants() {
    list::tests::_test_list_invariants(allocvec_backed_list::<()>(TEST_CAPACITY));
}

#[test]
fn test_allocvec_list_front_push_peek_pop_consistency() {
    list::tests::_test_list_front_push_peek_pop_consistency(allocvec_backed_list::<i32>(
        TEST_CAPACITY,
    ));
}

#[test]
fn test_allocvec_list_back_push_peek_pop_consistency() {
    list::tests::_test_list_back_push_peek_pop_consistency(allocvec_backed_list::<i32>(
        TEST_CAPACITY,
    ));
}

#[test]
fn test_allocvec_list_remove() {
    list::tests::_test_list_remove(allocvec_backed_list::<i32>(TEST_CAPACITY));
}

#[test]
fn test_allocvec_list_shift_push() {
    list::tests::_test_list_shift_push(allocvec_backed_list::<i32>(TEST_CAPACITY));
}

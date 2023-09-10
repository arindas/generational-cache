use generational_cache::{
    arena::{self, Arena, Entry},
    collections::list::{self, LinkedList, LinkedListArenaEntry},
    vector::{self, impls::alloc_vec::AllocVec},
};

const TEST_CAPACITY: usize = 1 << 4;

pub fn alloc_vec_backed_arena<T>(capacity: usize) -> Arena<AllocVec<Entry<T>>, T> {
    Arena::with_vector(AllocVec::with_capacity(capacity))
}

pub fn alloc_vec_backed_list<T>(
    capacity: usize,
) -> LinkedList<AllocVec<LinkedListArenaEntry<T>>, T> {
    LinkedList::with_backing_vector(AllocVec::with_capacity(capacity))
}

#[test]
fn test_alloc_vec_vector_consitency() {
    vector::tests::_test_vector_consistency(AllocVec::<usize>::with_capacity(TEST_CAPACITY));
}

#[test]
fn test_alloc_vec_arena_free_entries_init() {
    arena::tests::_test_arena_free_entries_init(alloc_vec_backed_arena::<()>(TEST_CAPACITY));
}

#[test]
fn test_alloc_vec_arena_reserve() {
    arena::tests::_test_arena_reserve(alloc_vec_backed_arena::<()>(TEST_CAPACITY));
}

#[test]
fn test_alloc_vec_arena_insert() {
    arena::tests::_test_arena_insert(alloc_vec_backed_arena::<i32>(TEST_CAPACITY));
}

#[test]
fn test_alloc_vec_arena_remove() {
    arena::tests::_test_arena_remove(alloc_vec_backed_arena::<i32>(TEST_CAPACITY));
}

#[test]
fn test_alloc_vec_list_invariants() {
    list::tests::_test_list_invariants(alloc_vec_backed_list::<()>(TEST_CAPACITY));
}

#[test]
fn test_alloc_vec_list_front_push_peek_pop_consistency() {
    list::tests::_test_list_front_push_peek_pop_consistency(alloc_vec_backed_list::<i32>(
        TEST_CAPACITY,
    ));
}

#[test]
fn test_alloc_vec_list_back_push_peek_pop_consistency() {
    list::tests::_test_list_back_push_peek_pop_consistency(alloc_vec_backed_list::<i32>(
        TEST_CAPACITY,
    ));
}

#[test]
fn test_alloc_vec_list_remove() {
    list::tests::_test_list_remove(alloc_vec_backed_list::<i32>(TEST_CAPACITY));
}

#[test]
fn test_alloc_vec_list_shift_push() {
    list::tests::_test_list_shift_push(alloc_vec_backed_list::<i32>(TEST_CAPACITY));
}

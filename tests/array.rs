use core::marker::Copy;
use generational_cache::{
    arena::{self, Arena, Entry},
    collections::list::{self, LinkedList, LinkedListArenaEntry},
    vector::{self, impls::array::Array},
};

const TEST_CAPACITY: usize = 1 << 4;

pub fn array_backed_arena<T, const N: usize>() -> Arena<Array<Entry<T>, N>, T>
where
    T: Copy + Default,
{
    Arena::with_vector(Array::new())
}

pub fn array_backed_list<T, const N: usize>() -> LinkedList<Array<LinkedListArenaEntry<T>, N>, T>
where
    T: Copy + Default,
{
    LinkedList::default()
}

#[test]
fn test_array_vector_consitency() {
    vector::tests::_test_vector_consistency(Array::<usize, TEST_CAPACITY>::new());
}

#[test]
fn test_array_arena_free_entries_init() {
    arena::tests::_test_arena_free_entries_init(array_backed_arena::<(), TEST_CAPACITY>());
}

#[test]
fn test_array_arena_insert() {
    arena::tests::_test_arena_insert(array_backed_arena::<i32, TEST_CAPACITY>());
}

#[test]
fn test_array_arena_remove() {
    arena::tests::_test_arena_remove(array_backed_arena::<i32, TEST_CAPACITY>());
}

#[test]
fn test_array_list_invariants() {
    list::tests::_test_list_invariants(array_backed_list::<(), TEST_CAPACITY>());
}

#[test]
fn test_array_list_front_push_peek_pop_consistency() {
    list::tests::_test_list_front_push_peek_pop_consistency(
        array_backed_list::<i32, TEST_CAPACITY>(),
    );
}

#[test]
fn test_array_list_back_push_peek_pop_consistency() {
    list::tests::_test_list_back_push_peek_pop_consistency(
        array_backed_list::<i32, TEST_CAPACITY>(),
    );
}

#[test]
fn test_array_list_remove() {
    list::tests::_test_list_remove(array_backed_list::<i32, TEST_CAPACITY>());
}

#[test]
fn test_array_list_shift_push() {
    list::tests::_test_list_shift_push(array_backed_list::<i32, TEST_CAPACITY>());
}

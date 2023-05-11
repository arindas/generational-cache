use crate::{
    arena::{Arena, Entry},
    collections::list::{LinkedList, Node},
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

pub fn array_backed_arena<T, const N: usize>() -> Arena<Array<Entry<T>, N>, T>
where
    T: Copy + Default,
{
    Arena::with_vector(Array::<Entry<T>, N>::new())
}

pub fn array_backed_list<T, const N: usize>() -> LinkedList<Array<Entry<Node<T>>, N>, T>
where
    T: Copy + Default,
{
    LinkedList::with_backing_arena(array_backed_arena())
}

#[cfg(test)]
mod tests {

    use super::{array_backed_arena, array_backed_list, Array};
    use crate::{arena, collections::list, vector};

    const TEST_CAPACITY: usize = 1 << 4;

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
        list::tests::_test_list_front_push_peek_pop_consistency(array_backed_list::<
            i32,
            TEST_CAPACITY,
        >());
    }

    #[test]
    fn test_array_list_back_push_peek_pop_consistency() {
        list::tests::_test_list_back_push_peek_pop_consistency(array_backed_list::<
            i32,
            TEST_CAPACITY,
        >());
    }

    #[test]
    fn test_array_list_remove() {
        list::tests::_test_list_remove(array_backed_list::<i32, TEST_CAPACITY>());
    }

    #[test]
    fn test_array_list_shift_push() {
        list::tests::_test_list_shift_push(array_backed_list::<i32, TEST_CAPACITY>());
    }
}

use super::super::Vector;
use core::marker::Copy;
use core::ops::{Deref, DerefMut};

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
}

pub mod arena_buffer {
    use crate::arena::Entry;
    pub type Array<T, const N: usize> = super::Array<Entry<T>, N>;
}

#[cfg(test)]
mod tests {
    use super::{
        super::super::super::{
            super::collections::list::{self, Node},
            tests,
        },
        arena_buffer::Array,
    };

    const TEST_CAPACITY: usize = 1 << 4;

    #[test]
    fn test_array_arena_free_entries_init() {
        tests::_test_arena_free_entries_init(TEST_CAPACITY, |_| Array::<(), TEST_CAPACITY>::new());
    }

    #[test]
    fn test_array_arena_insert() {
        tests::_test_arena_insert(TEST_CAPACITY, |_| Array::<i32, TEST_CAPACITY>::new());
    }

    #[test]
    fn test_array_arena_remove() {
        tests::_test_arena_remove(TEST_CAPACITY, |_| Array::<i32, TEST_CAPACITY>::new());
    }

    #[test]
    fn test_array_list_invariants() {
        list::tests::_test_list_invariants(TEST_CAPACITY, |_| {
            Array::<Node<()>, TEST_CAPACITY>::new()
        });
    }
}

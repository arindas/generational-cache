use crate::vector::Vector;
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

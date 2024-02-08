//! Module provinding a vector implementation based on arrays.

use crate::vector::Vector;
use core::{
    marker::Copy,
    ops::{Deref, DerefMut},
};

/// Implements [`Vector`] with `[T; N]`.
pub struct Array<T, const N: usize> {
    buffer: [T; N],
    len: usize,
}

impl<T, const N: usize> Array<T, N> {
    /// Creates an [`Array`] with the given buffer.
    pub fn with_buffer(buffer: [T; N]) -> Self {
        Self { buffer, len: 0 }
    }
}

impl<T, const N: usize> Array<T, N>
where
    T: Copy + Default,
{
    /// Creates a new [`Array`].
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

/// Error assocaited with operations on [`Array`]
#[derive(PartialEq, Debug)]
pub enum ArrayError {
    /// Used when attempting to push en element into an [`Array`] when it's at capacity.
    OutOfMemory,
}

impl<T, const N: usize> Vector<T> for Array<T, N> {
    type Error = ArrayError;

    fn reserve(&mut self, additional: usize) -> Result<(), Self::Error> {
        let remaining = self.capacity() - self.len();

        if additional > remaining {
            Err(ArrayError::OutOfMemory)
        } else {
            Ok(())
        }
    }

    fn capacity(&self) -> usize {
        N
    }

    fn push(&mut self, item: T) -> Result<(), Self::Error> {
        if self.len() == self.capacity() {
            Err(Self::Error::OutOfMemory)
        } else {
            self.buffer[self.len] = item;
            self.len += 1;
            Ok(())
        }
    }

    fn clear(&mut self) {
        self.len = 0;
    }
}

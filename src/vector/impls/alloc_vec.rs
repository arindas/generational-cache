//! Module providing a vector implementation based on [alloc::vec::Vec].

extern crate alloc;

use crate::vector::Vector;
use alloc::vec::Vec;
use core::{
    convert::Infallible,
    ops::{Deref, DerefMut},
};

/// Implements [Vector] with [alloc::vec::Vec].
pub struct AllocVec<T> {
    vec: Vec<T>,
}

impl<T> Default for AllocVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AllocVec<T> {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
        }
    }
}

impl<T> DerefMut for AllocVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec[..]
    }
}

impl<T> Deref for AllocVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.vec[..]
    }
}

impl<T> Vector<T> for AllocVec<T> {
    type Error = Infallible;

    fn reserve(&mut self, additional: usize) -> Result<(), Self::Error> {
        self.vec.reserve_exact(additional);
        Ok(())
    }

    fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    fn push(&mut self, item: T) -> Result<(), Self::Error> {
        self.vec.push(item);
        Ok(())
    }

    fn clear(&mut self) {
        self.vec.clear()
    }
}

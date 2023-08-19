extern crate alloc;

use crate::vector::Vector;
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};

#[allow(unused)]
pub struct AllocVec<T> {
    vec: Vec<T>,
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
    fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    fn push(&mut self, item: T) {
        self.vec.push(item)
    }

    fn clear(&mut self) {
        self.vec.clear()
    }
}

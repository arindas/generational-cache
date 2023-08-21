//! Module providing abstractions for reperesenting vectors.

use core::ops::DerefMut;

/// Represents an abstract vector over a type accessible as mutable slice.
pub trait Vector<T>: DerefMut<Target = [T]> {
    type Error: core::fmt::Debug;

    fn capacity(&self) -> usize;

    fn push(&mut self, item: T) -> Result<(), Self::Error>;

    fn clear(&mut self);
}

pub mod impls;

#[doc(hidden)]
pub mod tests {
    use super::Vector;

    pub fn _test_vector_consistency<V: Vector<usize>>(mut vector: V) {
        vector.clear();

        assert!(vector.is_empty());

        for i in 0..vector.capacity() {
            assert!(matches!(vector.push(i), Ok(_)));
        }

        assert_eq!(vector.len(), vector.capacity());

        for (j, i) in vector.iter().enumerate() {
            assert_eq!(i, &j);
        }

        vector.clear();

        assert!(vector.is_empty());
    }
}

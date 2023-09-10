//! Module providing abstractions for reperesenting vectors.

use core::ops::DerefMut;

/// Represents an abstract vector over a type accessible as mutable slice.
pub trait Vector<T>: DerefMut<Target = [T]> {
    type Error: core::fmt::Debug;

    fn reserve(&mut self, additional: usize) -> Result<(), Self::Error>;

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

        let cap_0 = vector.capacity();

        vector.reserve(vector.capacity()).unwrap();

        let cap_1 = vector.capacity();

        assert_eq!(cap_0, cap_1);

        assert!(vector.is_empty());

        for i in 0..vector.capacity() {
            vector.push(i).unwrap();
        }

        assert_eq!(vector.len(), vector.capacity());

        for (j, i) in vector.iter().enumerate() {
            assert_eq!(i, &j);
        }

        let cap_0 = vector.capacity();

        let res = vector.push(42);

        let cap_1 = vector.capacity();

        if cap_0 == cap_1 {
            assert!(res.is_err());
        } else {
            assert!(cap_1 > cap_0, "Capacity decreased on push().");
            assert!(res.is_ok());
        }

        let cap_0 = vector.capacity();

        vector.clear();

        let cap_1 = vector.capacity();

        assert!(cap_0 == cap_1, "Capacity changed on clear().");

        assert!(vector.is_empty());

        const ADDITIONAL: usize = 5;

        let result = vector.reserve(ADDITIONAL);

        if result.is_err() {
            return;
        }

        for i in 0..ADDITIONAL {
            vector.push(i).unwrap();
        }
    }
}

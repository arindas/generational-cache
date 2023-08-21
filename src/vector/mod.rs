use core::ops::DerefMut;

pub trait Vector<T>: DerefMut<Target = [T]> {
    fn capacity(&self) -> usize;

    fn push(&mut self, item: T);

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
            vector.push(i);
        }

        assert_eq!(vector.len(), vector.capacity());

        for (j, i) in vector.iter().enumerate() {
            assert_eq!(i, &j);
        }

        vector.clear();

        assert!(vector.is_empty());
    }
}

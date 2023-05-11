use core::ops::DerefMut;

pub trait Vector<T>: DerefMut<Target = [T]> {
    fn capacity(&self) -> usize;

    fn push(&mut self, item: T);

    fn clear(&mut self);
}

pub(crate) mod tests {
    use super::Vector;

    pub(crate) fn _test_vector_consistency<V: Vector<usize>>(mut vector: V) {
        vector.clear();

        assert!(vector.is_empty());

        for i in 0..vector.capacity() {
            vector.push(i);
        }

        assert_eq!(vector.len(), vector.capacity());

        let mut j = 0;

        for i in vector.iter() {
            assert_eq!(i, &j);
            j += 1;
        }

        vector.clear();

        assert!(vector.is_empty());
    }
}

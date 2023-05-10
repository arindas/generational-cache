use core::ops::DerefMut;

pub trait Vector<T>: DerefMut<Target = [T]> {
    fn capacity(&self) -> usize;

    fn push(&mut self, item: T);
}

pub mod impls;

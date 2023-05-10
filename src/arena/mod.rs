//! Module providing abstractions for a generational arena implemenation.

pub mod vector;

use core::marker::PhantomData;
use vector::Vector;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Index {
    pub generation: u64,
    pub idx: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Entry<T> {
    Occupied { value: T, generation: u64 },
    Free { next_free_idx: Option<usize> },
    Unmapped,
}

impl<T> Default for Entry<T> {
    fn default() -> Self {
        Self::Unmapped
    }
}

#[allow(unused)]
pub struct Arena<V, T> {
    entries_vec: V,
    generation: u64,
    free_list_head: Option<usize>,

    len: usize,

    _phantom_type: PhantomData<T>,
}

impl<V, T> Arena<V, T>
where
    V: Vector<Entry<T>>,
{
    pub fn with_vector(mut vector: V) -> Self {
        let capacity = vector.capacity();

        for i in 0..capacity {
            let next_free_idx = i + 1;
            let next_free_idx = if next_free_idx < capacity {
                Some(next_free_idx)
            } else {
                None
            };
            vector.push(Entry::Free { next_free_idx });
        }

        Self {
            entries_vec: vector,
            generation: 0,
            free_list_head: Some(0),
            len: 0,
            _phantom_type: PhantomData,
        }
    }

    pub fn insert(&mut self, item: T) -> Result<Index, ()> {
        let old_free = self.free_list_head.ok_or(())?;

        self.free_list_head = self
            .entries_vec
            .get(old_free)
            .map(|x| match x {
                Entry::Free { next_free_idx } => *next_free_idx,
                _ => None,
            })
            .ok_or(())?;

        let entry = Entry::Occupied {
            value: item,
            generation: self.generation,
        };

        *self.entries_vec.get_mut(old_free).ok_or(())? = entry;
        self.generation += 1;

        self.len += 1;

        Ok(Index {
            generation: self.generation - 1,
            idx: old_free,
        })
    }

    pub fn remove(&mut self, index: &Index) -> Option<T> {
        match self.entries_vec.get(index.idx) {
            Some(Entry::Occupied {
                value: _,
                generation,
            }) if &index.generation == generation => {
                let new_free_list_head_entry = Entry::<T>::Free {
                    next_free_idx: self.free_list_head,
                };

                let old_entry = core::mem::replace(
                    self.entries_vec.get_mut(index.idx)?,
                    new_free_list_head_entry,
                );

                self.free_list_head = Some(index.idx);

                self.len -= 1;

                Some(old_entry)
            }
            _ => None,
        }
        .and_then(|x| match x {
            Entry::Occupied {
                value,
                generation: _,
            } => Some(value),
            _ => None,
        })
    }

    pub fn get_mut(&mut self, index: &Index) -> Option<&mut T> {
        match self.entries_vec.get_mut(index.idx) {
            Some(Entry::Occupied { value, generation }) if &index.generation == generation => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn get(&mut self, index: &Index) -> Option<&T> {
        match self.entries_vec.get(index.idx) {
            Some(Entry::Occupied { value, generation }) if &index.generation == generation => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn capacity(&self) -> usize {
        self.entries_vec.capacity()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

pub(crate) mod tests {
    use super::{Arena, Entry, Index, Vector};
    use core::{cmp::PartialEq, fmt::Debug};

    pub(crate) fn _test_arena_free_entries_init<T, V, VP>(test_capacity: usize, vec_provider: VP)
    where
        V: Vector<Entry<T>>,
        VP: Fn(usize) -> V,
        T: Debug + PartialEq,
    {
        let arena = Arena::with_vector(vec_provider(test_capacity));

        assert_eq!(arena.capacity(), test_capacity);

        assert_eq!(arena.free_list_head, Some(0));

        for i in 0..test_capacity {
            let entry = &arena.entries_vec[i];

            if i == test_capacity - 1 {
                assert_eq!(
                    entry,
                    &Entry::Free {
                        next_free_idx: None
                    }
                )
            } else {
                assert_eq!(
                    entry,
                    &Entry::Free {
                        next_free_idx: Some(i + 1)
                    }
                )
            };
        }
    }

    pub(crate) fn _test_arena_insert<V, VP>(test_capacity: usize, vec_provider: VP)
    where
        V: Vector<Entry<i32>>,
        VP: Fn(usize) -> V,
    {
        assert!(
            test_capacity >= 2,
            "Test not valid for arena with capacity < 2"
        );

        let mut arena = Arena::with_vector(vec_provider(test_capacity));

        let index_0 = arena.insert(0);
        assert_eq!(
            index_0,
            Ok(Index {
                generation: 0,
                idx: 0
            })
        );

        let index_1 = arena.insert(1);
        assert_eq!(
            index_1,
            Ok(Index {
                generation: 1,
                idx: 1
            })
        );

        let index_0_val = index_0.unwrap();
        let item_0 = arena.get(&index_0_val);
        assert_eq!(item_0, Some(&0));

        let index_1_val = index_1.unwrap();
        let item_1 = arena.get(&index_1_val);
        assert_eq!(item_1, Some(&1));

        let item_0 = arena.get_mut(&index_0_val);
        assert_eq!(item_0, Some(&mut 0));
        let item_0 = item_0.unwrap();
        *item_0 = 25;

        let item_0 = arena.get(&index_0_val);
        assert_eq!(item_0, Some(&25));

        let item_1 = arena.get_mut(&index_1_val);
        assert_eq!(item_1, Some(&mut 1));
        let item_1 = item_1.unwrap();
        *item_1 = -78;

        let item_1 = arena.get(&index_1_val);
        assert_eq!(item_1, Some(&-78));

        let last_len = arena.len();

        let remaining = arena.capacity() - last_len;

        for i in 0..remaining {
            let possible_idx = last_len + i;

            assert_eq!(
                arena.insert(0),
                Ok(Index {
                    generation: possible_idx as u64,
                    idx: possible_idx
                })
            )
        }
    }

    pub(crate) fn _test_arena_remove<V, VP>(test_capacity: usize, vec_provider: VP)
    where
        V: Vector<Entry<i32>>,
        VP: Fn(usize) -> V,
    {
        assert!(
            test_capacity >= 2,
            "Test not valid for arena with capacity < 2"
        );

        let mut arena = Arena::with_vector(vec_provider(test_capacity));

        assert_eq!(arena.free_list_head.unwrap(), 0);

        let index = arena.insert(0).unwrap();
        assert_eq!(arena.get(&index), Some(&0));
        assert_eq!(
            index,
            Index {
                generation: 0,
                idx: 0
            }
        );

        assert_eq!(arena.free_list_head.unwrap(), 1);

        assert_eq!(arena.remove(&index).unwrap(), 0);
        assert_eq!(arena.get(&index), None);

        assert_eq!(arena.free_list_head.unwrap(), 0);

        let index = arena.insert(0).unwrap();
        assert_eq!(arena.get(&index), Some(&0));
        assert_eq!(
            index,
            Index {
                generation: 1,
                idx: 0
            }
        );

        assert_eq!(arena.free_list_head.unwrap(), 1);

        let last_arena_len = arena.len();
        let remaining = arena.capacity() - last_arena_len;

        let current_generation = index.generation + 1;

        for i in 0..remaining {
            let index = arena.insert(i as i32).unwrap();
            assert_eq!(
                index,
                Index {
                    generation: current_generation + i as u64,
                    idx: last_arena_len + i
                }
            );
        }

        // remove elements at odd indices
        let mut i = 1;
        let mut removed_count = 0;
        while i < arena.capacity() {
            arena
                .remove(&Index {
                    generation: i as u64 + 1,
                    idx: i,
                })
                .unwrap();

            i += 2;
            removed_count += 1;
        }

        // iterate through free list
        let mut free_position_count = 0;
        let mut free_idx = arena.free_list_head;

        while let Some(next_free) = free_idx {
            assert_eq!(next_free & 1, 1);
            free_idx = match arena.entries_vec[next_free] {
                Entry::Free { next_free_idx } => next_free_idx,
                _ => None,
            };
            free_position_count += 1;
        }

        assert_eq!(removed_count, free_position_count);
    }
}

//! Module providing abstractions for a linked list implementation.

use core::fmt::{self, Debug, Display};

use crate::{
    arena::{Arena, ArenaError, Entry, Index},
    vector::Vector,
};

/// Represents a link to node in the linked list.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Link {
    pub index: Index,
}

/// Represents a node in a linked list.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Node<T> {
    pub value: T,

    pub next: Option<Link>,
    pub prev: Option<Link>,
}

impl<T> Node<T> {
    pub fn with_value(value: T) -> Self {
        Self {
            value,
            next: None,
            prev: None,
        }
    }
}

impl<T> Default for Node<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            value: Default::default(),
            next: Default::default(),
            prev: Default::default(),
        }
    }
}

/// A double-linked linked list implementation using a generational [`Arena`] for allocation.
pub struct LinkedList<V, T> {
    backing_arena: Arena<V, Node<T>>,

    head: Option<Link>,
    tail: Option<Link>,

    len: usize,
}

/// Error type associated with list operations.
#[derive(Debug)]
pub enum ListError<VE> {
    /// Used when there is an error in an operation performed on the underlying arena.
    ArenaError(ArenaError<VE>),

    /// Used when a link is not associated with a node in the underlying arena.
    LinkBroken,

    /// Used when attempting to remove items from an empty list.
    ListEmpty,
}

impl<VE> Display for ListError<VE>
where
    VE: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Type alias for arena entries corresponding to [`LinkedList`] [`Node`] instances.
pub type LinkedListArenaEntry<T> = Entry<Node<T>>;

impl<V, T> LinkedList<V, T>
where
    V: Vector<Entry<Node<T>>>,
{
    /// Creates a new [`LinkedList`] with given the backing [`Vector`] for the underlying [`Arena`].
    pub fn with_backing_vector(vector: V) -> Self {
        Self {
            backing_arena: Arena::with_vector(vector),
            head: None,
            tail: None,
            len: 0,
        }
    }

    /// Removes all elements from this [`LinkedList`].
    pub fn clear(&mut self) -> Result<(), ListError<V::Error>> {
        self.backing_arena.clear().map_err(ListError::ArenaError)?;

        self.head = None;
        self.tail = None;
        self.len = 0;

        Ok(())
    }

    /// Reserves memory for the give number of additional elements in this [`LinkedList`].
    pub fn reserve(&mut self, additional: usize) -> Result<(), ListError<V::Error>> {
        let remaining = self.capacity() - self.len();

        if remaining >= additional {
            return Ok(());
        }

        self.backing_arena
            .reserve(additional)
            .map_err(ListError::ArenaError)
    }

    /// Returns the number of elements this [`LinkedList`] is capable of storing.
    ///
    /// Since this [`LinkedList`] uses an [`Arena`] for allocation, it's capacity is subject to the
    /// capacity of the underlying [`Arena`].
    pub fn capacity(&self) -> usize {
        self.backing_arena.capacity()
    }

    /// Returns the number of elements stored in this [`LinkedList`].
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns whether this [`LinkedList`] is empty.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Returns a mutable reference to the [`Node`] referenced by the given [`Link`].
    fn get_node_mut(&mut self, link: &Link) -> Option<&mut Node<T>> {
        self.backing_arena.get_mut(&link.index)
    }

    /// Returns an immutable reference to the [`Node`] referenced by the given [`Link`].
    fn get_node(&self, link: &Link) -> Option<&Node<T>> {
        self.backing_arena.get(&link.index)
    }

    /// Returns a mutable reference to the element stored in the [`Node`] at the given [`Link`].
    pub fn get_mut(&mut self, link: &Link) -> Option<&mut T> {
        Some(&mut self.get_node_mut(link)?.value)
    }

    /// Returns an imutable reference to the element stored in the [`Node`] at the given [`Link`].
    pub fn get(&self, link: &Link) -> Option<&T> {
        Some(&self.get_node(link)?.value)
    }

    fn link_head(&mut self, link: Link) -> Option<()> {
        self.get_node_mut(&link)?.next = self.head;

        if let Some(head_link) = self.head {
            self.get_node_mut(&head_link)?.prev = Some(link);
        } else {
            self.tail = Some(link);
        }

        self.head = Some(link);

        self.len += 1;

        Some(())
    }

    fn link_tail(&mut self, link: Link) -> Option<()> {
        self.get_node_mut(&link)?.prev = self.tail;

        if let Some(tail_link) = self.tail {
            self.get_node_mut(&tail_link)?.next = Some(link);
        } else {
            self.head = Some(link);
        }

        self.tail = Some(link);

        self.len += 1;

        Some(())
    }

    /// Pushes the given element to the front of this [`LinkedList`].
    pub fn push_front(&mut self, value: T) -> Result<Link, ListError<V::Error>> {
        let node_index = self
            .backing_arena
            .insert(Node::with_value(value))
            .map_err(ListError::ArenaError)?;

        let node_link = Link { index: node_index };

        self.link_head(node_link).ok_or(ListError::LinkBroken)?;

        Ok(node_link)
    }

    /// Pushes the given element to the back of this [`LinkedList`].
    pub fn push_back(&mut self, value: T) -> Result<Link, ListError<V::Error>> {
        let node_index = self
            .backing_arena
            .insert(Node::with_value(value))
            .map_err(ListError::ArenaError)?;

        let node_link = Link { index: node_index };

        self.link_tail(node_link).ok_or(ListError::LinkBroken)?;

        Ok(node_link)
    }

    /// Peeks the element at the front of this list.
    pub fn peek_front(&self) -> Option<&T> {
        self.get(self.head.as_ref()?)
    }

    /// Peeks the element at the back of this list.
    pub fn peek_back(&self) -> Option<&T> {
        self.get(self.tail.as_ref()?)
    }

    fn unlink_head(&mut self) -> Option<Link> {
        let head_link = self.head?;
        self.head = self.get_node(&head_link)?.next;

        let to_unlink = match self.head {
            Some(new_head_link) => &mut self.get_node_mut(&new_head_link)?.prev,
            None => &mut self.tail,
        };

        *to_unlink = None;

        self.len -= 1;

        Some(head_link)
    }

    fn unlink_tail(&mut self) -> Option<Link> {
        let tail_link = self.tail?;
        self.tail = self.get_node(&tail_link)?.prev;

        let to_unlink = match self.tail {
            Some(new_tail_link) => &mut self.get_node_mut(&new_tail_link)?.next,
            None => &mut self.head,
        };

        *to_unlink = None;

        self.len -= 1;

        Some(tail_link)
    }

    fn unlink(&mut self, link: &Link) -> Option<Link> {
        match Some(link) {
            link if link == self.head.as_ref() => self.unlink_head(),
            link if link == self.tail.as_ref() => self.unlink_tail(),
            _ => {
                let node = self.get_node_mut(link)?;

                let prev_link = node.prev?;
                let next_link = node.next?;

                node.next = None;
                node.prev = None;

                self.get_node_mut(&prev_link)?.next = Some(next_link);
                self.get_node_mut(&next_link)?.prev = Some(prev_link);

                self.len -= 1;

                Some(*link)
            }
        }
    }

    fn reclaim(&mut self, link: &Link) -> Option<T> {
        let node = self.backing_arena.remove(&link.index)?;
        Some(node.value)
    }

    /// Removes the element referenced by the given link.
    pub fn remove(&mut self, link: &Link) -> Option<T> {
        let link = self.unlink(link)?;
        self.reclaim(&link)
    }

    /// Removes the element at the front of this list.
    pub fn pop_front(&mut self) -> Option<T> {
        let link = self.unlink_head()?;
        self.reclaim(&link)
    }

    /// Removes the element at the back of this list.
    pub fn pop_back(&mut self) -> Option<T> {
        let link = self.unlink_tail()?;
        self.reclaim(&link)
    }

    /// Shifts the element at the given [`Link`] to the front of this list.
    pub fn shift_push_front(&mut self, link: &Link) -> Option<()> {
        let link = self.unlink(link)?;
        self.link_head(link)
    }

    /// Shifts the element at the given [`Link`] to the back of this list.
    pub fn shift_push_back(&mut self, link: &Link) -> Option<()> {
        let link = self.unlink(link)?;
        self.link_tail(link)
    }

    /// Returns an iterator to iterate over the elements in this list.
    pub fn iter(&self) -> Iter<'_, V, T> {
        Iter {
            list: self,
            cursor: self.head.as_ref(),
        }
    }
}

impl<V, T> Default for LinkedList<V, T>
where
    V: Default + Vector<Entry<Node<T>>>,
{
    fn default() -> Self {
        Self::with_backing_vector(V::default())
    }
}

/// Iterator implementation to iterate over the items in a [`LinkedList`].
pub struct Iter<'a, V, T> {
    list: &'a LinkedList<V, T>,
    cursor: Option<&'a Link>,
}

impl<'a, V, T> Iterator for Iter<'a, V, T>
where
    V: Vector<Entry<Node<T>>>,
{
    type Item = (&'a Link, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let cursor = self.cursor.take()?;
        let cursor_node = self.list.get_node(cursor)?;

        self.cursor = cursor_node.next.as_ref();

        Some((cursor, &cursor_node.value))
    }
}

impl<'a, V, T> IntoIterator for &'a LinkedList<V, T>
where
    V: Vector<Entry<Node<T>>>,
{
    type Item = (&'a Link, &'a T);

    type IntoIter = Iter<'a, V, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[doc(hidden)]
pub mod tests {
    use super::{
        super::super::{
            arena::{ArenaError, Entry},
            collections::list::ListError,
            vector::Vector,
        },
        LinkedList, Node,
    };
    use core::fmt::Debug;

    pub fn _test_list_invariants<T, V>(mut list: LinkedList<V, T>)
    where
        V: Vector<Entry<Node<T>>>,
        T: Debug + PartialEq + Default,
    {
        list.clear().unwrap();

        let capacity = list.capacity();

        assert!(list.is_empty());

        assert_eq!(list.peek_front(), None);
        assert_eq!(list.peek_back(), None);

        for _ in 0..capacity {
            list.push_back(T::default()).unwrap();
        }

        assert!(list.len() == list.capacity());

        let mut i = 0;
        for (_, t) in &list {
            assert_eq!(t, &T::default());
            i += 1;
        }

        assert_eq!(i, list.len());

        assert_eq!(list.peek_front(), Some(&T::default()));
        assert_eq!(list.peek_back(), Some(&T::default()));

        match list.push_front(T::default()) {
            Err(ListError::ArenaError(ArenaError::OutOfMemory)) => {}
            _ => unreachable!("Out of memory not triggered"),
        };

        match list.push_back(T::default()) {
            Err(ListError::ArenaError(ArenaError::OutOfMemory)) => {}
            _ => unreachable!("Out of memory not triggered"),
        };

        const ADDITIONAL: usize = 5;

        let result = list.reserve(ADDITIONAL);

        for _ in 0..ADDITIONAL {
            if result.is_ok() {
                list.push_front(T::default()).unwrap();
            }
        }

        let result = list.reserve(ADDITIONAL);

        for _ in 0..ADDITIONAL {
            if result.is_ok() {
                list.push_front(T::default()).unwrap();
            }
        }

        list.clear().unwrap();

        assert!(list.is_empty());
    }

    pub fn _test_list_front_push_peek_pop_consistency<V>(mut list: LinkedList<V, i32>)
    where
        V: Vector<Entry<Node<i32>>>,
    {
        list.clear().unwrap();

        let capacity = list.capacity();

        assert!(list.is_empty());
        assert_eq!(list.peek_front(), None);
        assert_eq!(list.pop_front(), None);

        for ele in 0..capacity {
            list.push_front(ele as i32).unwrap();
        }

        match list.push_front(0) {
            Err(ListError::ArenaError(ArenaError::OutOfMemory)) => {}
            _ => unreachable!("Out of memory not triggered"),
        };

        assert_eq!(list.peek_front().unwrap(), &(capacity as i32 - 1));

        let mut i = capacity as i32 - 1;
        for (_, ele) in &list {
            assert_eq!(ele, &i);
            i -= 1;
        }
        assert_eq!(i, -1);

        let mut i = capacity as i32 - 1;
        while let Some(ele) = list.pop_front() {
            assert_eq!(ele, i);
            i -= 1;
        }
        assert_eq!(i, -1);

        assert!(list.is_empty());
    }

    pub fn _test_list_back_push_peek_pop_consistency<V>(mut list: LinkedList<V, i32>)
    where
        V: Vector<Entry<Node<i32>>>,
    {
        list.clear().unwrap();

        let capacity = list.capacity();

        assert!(list.is_empty());
        assert_eq!(list.peek_back(), None);
        assert_eq!(list.pop_back(), None);

        for ele in 0..capacity {
            list.push_back(ele as i32).unwrap();
        }

        match list.push_back(0) {
            Err(ListError::ArenaError(ArenaError::OutOfMemory)) => {}
            _ => unreachable!("Out of memory not triggered"),
        };

        assert_eq!(list.peek_back().unwrap(), &(capacity as i32 - 1));

        let mut i = 0;
        for (_, ele) in &list {
            assert_eq!(ele, &i);
            i += 1;
        }
        assert_eq!(i as usize, capacity);

        let mut i = capacity as i32 - 1;
        while let Some(ele) = list.pop_back() {
            assert_eq!(ele, i);
            i -= 1;
        }
        assert_eq!(i, -1);

        assert!(list.is_empty());
    }

    pub fn _test_list_remove<V>(mut list: LinkedList<V, i32>)
    where
        V: Vector<Entry<Node<i32>>>,
    {
        let capacity = list.capacity();

        assert!(capacity >= 3, "Test not valid for lists with capacity < 3 ");

        list.clear().unwrap();
        assert!(list.is_empty());

        for ele in 0..capacity {
            list.push_back(ele as i32).unwrap();
        }

        let link = *list.iter().find(|&(_, value)| value & 1 == 1).unwrap().0;

        list.remove(&link).unwrap();

        assert!(list.remove(&link).is_none());

        assert!(list.get(&link).is_none());

        assert_eq!(list.len(), list.capacity() - 1);

        for (_, ele) in &list {
            assert_ne!(ele, &1);
        }

        let link = *list.iter().find(|&(_, value)| value & 1 == 0).unwrap().0;

        list.remove(&link).unwrap();

        assert_eq!(list.peek_front(), Some(&2));

        assert_eq!(list.len(), list.capacity() - 2);

        let mut link = None;

        for (l, _) in &list {
            link = Some(l);
        }

        let link = *link.unwrap();

        list.remove(&link).unwrap();

        assert_eq!(list.len(), list.capacity() - 3);
    }

    pub fn _test_list_shift_push<V>(mut list: LinkedList<V, i32>)
    where
        V: Vector<Entry<Node<i32>>>,
    {
        let capacity = list.capacity();

        assert!(capacity >= 3, "Test not valid for lists with capacity < 3 ");

        list.clear().unwrap();
        assert!(list.is_empty());

        for ele in 0..capacity {
            list.push_back(ele as i32).unwrap();
        }

        assert_eq!(list.peek_front(), Some(&0));

        let link = *list.iter().find(|&(_, value)| value & 1 == 1).unwrap().0;

        assert_eq!(list.len(), list.capacity());

        list.shift_push_front(&link).unwrap();

        assert_eq!(list.len(), list.capacity());

        assert_eq!(list.peek_front(), Some(&1));

        for (i, j) in list
            .iter()
            .take(3)
            .map(|(_, value)| value)
            .zip([1, 0, 2].iter())
        {
            assert_eq!(i, j);
        }

        let link = *list.iter().find(|&(_, value)| value & 1 == 0).unwrap().0;

        assert_eq!(list.get(&link), Some(&0));

        assert_ne!(list.peek_back(), Some(&0));

        assert_eq!(list.len(), list.capacity());

        list.shift_push_back(&link).unwrap();

        assert_eq!(list.peek_back(), Some(&0));

        assert_eq!(list.len(), list.capacity());
    }
}

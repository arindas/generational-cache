//! Module providing abstractions for a linked list implementation.

use core::fmt::{self, Display};

use crate::arena::{vector::Vector, Arena, ArenaError, Entry, Index};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Link {
    pub index: Index,
}

#[allow(unused)]
pub struct Node<T> {
    pub value: T,

    pub next: Option<Link>,
    pub prev: Option<Link>,
}

#[allow(unused)]
pub struct LinkedList<V, T> {
    backing_arena: Arena<V, Node<T>>,

    head: Option<Link>,
    tail: Option<Link>,

    len: usize,
}

#[allow(unused)]
pub struct Iter<'a, V, T> {
    list: &'a LinkedList<V, T>,
    current: Option<Link>,
}

#[derive(PartialEq, Debug)]
pub enum ListError {
    ArenaError(ArenaError),
    LinkBroken,
    ListEmpty,
}

impl Display for ListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[allow(unused)]
impl<V, T> LinkedList<V, T>
where
    V: Vector<Entry<Node<T>>>,
{
    pub fn with_vector(vector: V) -> Self {
        Self {
            backing_arena: Arena::with_vector(vector),
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.backing_arena.capacity()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }

    fn get_mut_node(&mut self, link: &Link) -> Option<&mut Node<T>> {
        self.backing_arena.get_mut(&link.index)
    }

    fn get_node(&self, link: &Link) -> Option<&Node<T>> {
        self.backing_arena.get(&link.index)
    }

    pub fn get_mut(&mut self, link: &Link) -> Option<&mut T> {
        Some(&mut self.get_mut_node(link)?.value)
    }

    pub fn get(&self, link: &Link) -> Option<&T> {
        Some(&self.get_node(link)?.value)
    }

    fn link_head(&mut self, link: Link) -> Option<()> {
        if let Some(head_link) = self.head {
            self.get_mut_node(&head_link)?.prev = Some(link);
        } else {
            self.tail = Some(link);
        }

        self.head = Some(link);

        self.len += 1;

        Some(())
    }

    fn link_tail(&mut self, link: Link) -> Option<()> {
        if let Some(tail_link) = self.tail {
            self.get_mut_node(&tail_link)?.next = Some(link);
        } else {
            self.head = Some(link);
        }

        self.tail = Some(link);

        self.len += 1;

        Some(())
    }

    pub fn push_front(&mut self, value: T) -> Result<Link, ListError> {
        let node = Node {
            value,
            next: self.head,
            prev: None,
        };

        let node_index = self
            .backing_arena
            .insert(node)
            .map_err(ListError::ArenaError)?;

        let node_link = Link { index: node_index };

        self.link_head(node_link).ok_or(ListError::LinkBroken)?;

        Ok(node_link)
    }

    pub fn push_back(&mut self, value: T) -> Result<Link, ListError> {
        let node = Node {
            value,
            next: None,
            prev: self.tail,
        };

        let node_index = self
            .backing_arena
            .insert(node)
            .map_err(ListError::ArenaError)?;

        let node_link = Link { index: node_index };

        self.link_tail(node_link).ok_or(ListError::LinkBroken)?;

        Ok(node_link)
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.get(self.head.as_ref()?)
    }

    pub fn peek_back(&self) -> Option<&T> {
        self.get(self.tail.as_ref()?)
    }

    fn unlink_head(&mut self) -> Option<Link> {
        let head_link = self.head?;
        self.head = self.get_node(&head_link)?.next;

        let to_unlink = match self.head {
            Some(new_head_link) => &mut self.get_mut_node(&new_head_link)?.prev,
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
            Some(new_tail_link) => &mut self.get_mut_node(&new_tail_link)?.next,
            None => &mut self.head,
        };

        *to_unlink = None;

        self.len -= 1;

        Some(tail_link)
    }

    pub fn unlink(&mut self, link: &Link) -> Option<Link> {
        match Some(link) {
            link if link == self.head.as_ref() => self.unlink_head(),
            link if link == self.tail.as_ref() => self.unlink_tail(),
            _ => {
                let node = self.get_node(link)?;

                let prev_link = node.prev?;
                let next_link = node.next?;

                self.get_mut_node(&prev_link)?.next = Some(next_link);
                self.get_mut_node(&next_link)?.prev = Some(prev_link);

                self.len -= 1;

                Some(*link)
            }
        }
    }

    pub fn remove(&mut self, link: &Link) -> Option<T> {
        let node = self.backing_arena.remove(&link.index)?;
        Some(node.value)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let link = self.unlink_head()?;
        self.remove(&link)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let link = self.unlink_tail()?;
        self.remove(&link)
    }

    pub fn shift_push_front(&mut self, link: &Link) -> Option<()> {
        let link = self.unlink(link)?;
        self.link_head(link)
    }

    pub fn shift_push_back(&mut self, link: &Link) -> Option<()> {
        let link = self.unlink(link)?;
        self.link_tail(link)
    }
}

impl<V, T> Default for LinkedList<V, T>
where
    V: Default + Vector<Entry<Node<T>>>,
{
    fn default() -> Self {
        Self::with_vector(Default::default())
    }
}

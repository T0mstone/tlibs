//! This crate provides a `Vec`-like struct that cannot be empty

use std::marker::PhantomData;
use std::mem;
use std::num::NonZeroUsize;

mod private {
    use super::{HeadFirst, HeadLast};

    pub trait Sealed {}
    impl Sealed for HeadFirst {}
    impl Sealed for HeadLast {}
}

/// Specifies the location the head item has in relation to the rest
pub trait HeadLocation: self::private::Sealed {
    /// This constant can be used to write code generic over `HeadLocation`
    /// without explicit casing
    const HEAD_FIRST: bool;
}

/// The head item is in front of the rest (at index `0`)
pub enum HeadFirst {}
/// The head item is after the rest (at index `len-1`)
pub enum HeadLast {}

impl HeadLocation for HeadFirst {
    const HEAD_FIRST: bool = true;
}
impl HeadLocation for HeadLast {
    const HEAD_FIRST: bool = false;
}

/// A `Vec` that always has at least one element
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct NonemptyVec<T, H: HeadLocation = HeadFirst> {
    head: T,
    body: Vec<T>,
    _marker: PhantomData<H>,
}

#[allow(clippy::len_without_is_empty)]
impl<T, H: HeadLocation> NonemptyVec<T, H> {
    /// Creates a new `NonemptyVec` with one element
    pub fn new(head: T) -> Self {
        Self {
            head,
            body: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// The `head` is simply the element guaranteed to exist.
    /// It is not necessarily interpreted as the first item
    pub fn head(&self) -> &T {
        &self.head
    }

    /// Like `head` but mutable
    pub fn head_mut(&mut self) -> &mut T {
        &mut self.head
    }

    /// Consumes `self` and returns the head
    pub fn into_head(self) -> T {
        self.head
    }

    /// The `tail` is simply all elements that are not guaranteed to exist
    pub fn tail(&self) -> &[T] {
        &self.body
    }

    /// Like `tail` but mutable
    pub fn tail_mut(&mut self) -> &mut Vec<T> {
        &mut self.body
    }

    /// Consumes `self` and returns the tail
    pub fn into_tail(self) -> Vec<T> {
        self.body
    }

    /// Consumes `self` and returns the head and the tail
    pub fn into_head_tail(self) -> (T, Vec<T>) {
        (self.head, self.body)
    }

    /// Returns the length of `self`
    ///
    /// # Edge Case
    /// The length can be `usize::MAX + 1` - in that case, only `usize::MAX` is returned
    ///
    /// To circumvent this, you can work with `self.tail().len()`, which is always one less than `self.len()`
    pub fn len(&self) -> NonZeroUsize {
        // Saftey: as you can see, this is always at leats 1
        unsafe { NonZeroUsize::new_unchecked(self.body.len().saturating_add(1)) }
    }

    // section: head-location specific functions
    // the optimizer will optimize out the constant checks

    /// Pushes an element to the end of `self`
    pub fn push(&mut self, t: T) {
        if H::HEAD_FIRST {
            self.body.push(t);
        } else {
            self.body.push(mem::replace(&mut self.head, t));
        }
    }

    /// Removes the last element from `self`
    /// (unless `self` has only one item left), returns it.
    pub fn pop(&mut self) -> Option<T> {
        if H::HEAD_FIRST {
            self.body.pop()
        } else {
            self.body
                .pop()
                .map(|new_head| mem::replace(&mut self.head, new_head))
        }
    }

    /// Inserts an element at the specified index
    pub fn insert(&mut self, index: usize, element: T) {
        assert!(
            index <= self.len().get(),
            "insertion index (is {}) should be <= len (is {})",
            index,
            self.len()
        );
        if H::HEAD_FIRST {
            if index == 0 {
                let tmp = std::mem::replace(&mut self.head, element);
                self.body.insert(0, tmp);
            } else {
                self.body.insert(index - 1, element);
            }
        } else if index == self.len().get() {
            self.push(element);
        } else {
            self.body.insert(index, element);
        }
    }

    /// Removes an element from the specified index;
    /// Returns `None` if there is only one element left
    pub fn remove(&mut self, index: usize) -> Option<T> {
        assert!(
            index <= self.len().get(),
            "removal index (is {}) should be <= len (is {})",
            index,
            self.len()
        );
        if self.len().get() == 1 {
            return None;
        }
        Some(if H::HEAD_FIRST {
            if index == 0 {
                std::mem::replace(&mut self.head, self.body.remove(0))
            } else {
                self.body.remove(index - 1)
            }
        } else if index == self.len().get() {
            self.pop()?
        } else {
            self.body.remove(index)
        })
    }

    /// Creates a `Vec` from all its elements
    pub fn into_vec(mut self) -> Vec<T> {
        let mut res;
        if H::HEAD_FIRST {
            res = vec![self.head];
            res.append(&mut self.body);
        } else {
            res = self.body;
            res.push(self.head);
        }
        res
    }

    /// The first element
    pub fn first(&self) -> &T {
        if H::HEAD_FIRST {
            &self.head
        } else {
            self.body.first().unwrap_or(&self.head)
        }
    }

    /// The first element, mutable
    pub fn first_mut(&mut self) -> &mut T {
        if H::HEAD_FIRST {
            &mut self.head
        } else {
            self.body.first_mut().unwrap_or(&mut self.head)
        }
    }

    /// Returns the current first element, consuming `self`
    pub fn into_first(mut self) -> T {
        if H::HEAD_FIRST || self.body.is_empty() {
            self.head
        } else {
            self.body.remove(0)
        }
    }

    /// The last element
    pub fn last(&self) -> &T {
        if H::HEAD_FIRST {
            self.body.last().unwrap_or(&self.head)
        } else {
            &self.head
        }
    }

    /// The last element, mutable
    pub fn last_mut(&mut self) -> &mut T {
        if H::HEAD_FIRST {
            self.body.last_mut().unwrap_or(&mut self.head)
        } else {
            &mut self.head
        }
    }

    /// Returns the current last element, consuming `self`
    pub fn into_last(mut self) -> T {
        if H::HEAD_FIRST {
            self.body.pop().unwrap_or(self.head)
        } else {
            self.head
        }
    }
}

impl<T> IntoIterator for NonemptyVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}

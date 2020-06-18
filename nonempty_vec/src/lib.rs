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
pub trait HeadLocation: self::private::Sealed {}

/// The head item is in front of the rest (at index `0`)
pub enum HeadFirst {}
/// The head item is after the rest (at index `len-1`)
pub enum HeadLast {}

impl HeadLocation for HeadFirst {}
impl HeadLocation for HeadLast {}

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

    /// Consumes `self` and returns the head
    pub fn into_head(self) -> T {
        self.head
    }

    /// Returns the length of `self`
    ///
    /// # Edge Case
    /// The length can be `usize::MAX + 1` - in that case, only `usize::MAX` is returned
    pub fn len(&self) -> NonZeroUsize {
        // Saftey: as you can see, this is always at leats 1
        unsafe { NonZeroUsize::new_unchecked(self.body.len().saturating_add(1)) }
    }
}

impl<T> NonemptyVec<T, HeadLast> {
    /// Sets `self.head` to `t`, pushing the old `self.head` back by `1`
    pub fn push(&mut self, t: T) {
        self.body.push(mem::replace(&mut self.head, t));
    }

    /// Returns `self.head`, then sets `self.head` to the new last item.
    ///
    /// Returns `None` if there is only the head left
    pub fn pop(&mut self) -> Option<T> {
        self.body
            .pop()
            .map(|new_head| mem::replace(&mut self.head, new_head))
    }

    /// Returns the current last element, consuming `self`
    pub fn into_pop(mut self) -> T {
        self.pop().unwrap_or_else(|| self.into_head())
    }

    /// Creates a `Vec` from all its elements
    pub fn into_vec(self) -> Vec<T> {
        let mut res = self.body;
        res.push(self.head);
        res
    }
}

impl<T> NonemptyVec<T, HeadFirst> {
    /// Pushes an item to the end of `self`
    pub fn push(&mut self, t: T) {
        self.body.push(t)
    }

    /// Returns the last item in `self` (removing it)
    ///
    /// Returns `None` if there is only the head left
    pub fn pop(&mut self) -> Option<T> {
        self.body.pop()
    }

    /// Returns the current last element, consuming `self`
    pub fn into_pop(mut self) -> T {
        self.pop().unwrap_or_else(|| self.into_head())
    }

    /// Creates a `Vec` from all its elements
    pub fn into_vec(mut self) -> Vec<T> {
        let mut res = vec![self.head];
        res.append(&mut self.body);
        res
    }
}

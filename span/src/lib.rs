//! This crate provides the [`Span`](struct.Span.html) struct.
#![no_std]

use core::ops::{Index, IndexMut, Range};

/// A `Span` is basically like a `Range<usize>`
/// but it has some additional methods and functionality
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Span {
    start: usize,
    len: usize,
}

impl Span {
    /// Creates a new `Span` from a start position and a length
    #[inline]
    pub fn new(start: usize, len: usize) -> Self {
        Self { start, len }
    }

    /// Creates a new `Span` from a start position and a length
    #[inline]
    pub fn from_range(range: Range<usize>) -> Self {
        Self {
            start: range.start,
            len: range.len(),
        }
    }

    /// The start position
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    /// The length
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Determines wheter the `Span` is empty, i.e. of length 0
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// The end position (the one-after-end index)
    #[inline]
    pub fn end(&self) -> usize {
        self.start + self.len
    }

    /// Converts the `Span` to its equivalent `Range<usize>`
    #[inline]
    pub fn as_range(&self) -> Range<usize> {
        self.start..self.end()
    }

    /// Returns whether `other` is contained within `self` (this includes equality)
    #[inline]
    pub fn contains(&self, other: &Self) -> bool {
        other.start >= self.start && other.end() <= self.end()
    }

    /// Create a (the smallest) span that contains both `self` and `other`
    #[inline]
    pub fn join(self, other: Self) -> Self {
        let start = self.start.min(other.start);
        let end = self.end().max(other.end());
        Self {
            start,
            len: end - start,
        }
    }
}

impl Into<Range<usize>> for Span {
    #[inline]
    fn into(self) -> Range<usize> {
        self.as_range()
    }
}

impl From<Range<usize>> for Span {
    #[inline]
    fn from(r: Range<usize>) -> Self {
        Self::from_range(r)
    }
}

impl<T> Index<Span> for [T] {
    type Output = Self;

    #[inline]
    fn index(&self, index: Span) -> &Self::Output {
        self.index(index.as_range())
    }
}

impl<T> IndexMut<Span> for [T] {
    #[inline]
    fn index_mut(&mut self, index: Span) -> &mut Self::Output {
        self.index_mut(index.as_range())
    }
}

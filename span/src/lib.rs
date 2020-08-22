//! This crate provides the [`Span`](struct.Span.html) struct.
#![no_std]

use core::cmp::Ordering;
use core::ops::{Deref, DerefMut, Index, IndexMut, Range};

/// A `Span` is basically like a `Range<usize>`
/// but it has some additional methods and functionality
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Span {
    /// The starting position
    pub start: usize,
    /// The length
    pub len: usize,
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

    /// Create a (the smallest) span that contains both all elements in `iter` (returns `None` iff `iter` is empty)
    #[inline]
    pub fn join_all<I: IntoIterator<Item = Self>>(iter: I) -> Option<Self> {
        iter.into_iter().fold(None, |acc, x| match acc {
            None => Some(x),
            Some(l) => Some(Span::join(l, x)),
        })
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Spanned<T> {
    pub span: Span,
    pub inner: T,
}

impl<T> Spanned<T> {
    pub fn new(span: Span, inner: T) -> Self {
        Self { span, inner }
    }
}

impl<T: PartialOrd> PartialOrd for Spanned<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

// the redundant PartialOrd here is due to my IDE
impl<T: PartialOrd + Ord> Ord for Spanned<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

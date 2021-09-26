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
	pub const fn new(start: usize, len: usize) -> Self {
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
	pub const fn start(&self) -> usize {
		self.start
	}

	/// The length
	#[inline]
	pub const fn len(&self) -> usize {
		self.len
	}

	/// Determines wheter the `Span` is empty, i.e. of length 0
	#[inline]
	pub const fn is_empty(&self) -> bool {
		self.len == 0
	}

	/// The end position (the one-after-end index)
	#[inline]
	pub const fn end(&self) -> usize {
		self.start + self.len
	}

	/// Converts the `Span` to its equivalent `Range<usize>`
	#[inline]
	pub const fn as_range(&self) -> Range<usize> {
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

	/// Split the span at a middle point
	///
	/// Illustration: Split `Span::new(3, 4)` at `mid = 5`
	/// ```text
	/// 3[**|**]7 -> (3[**]5, 5[**]7)
	/// ```
	///
	/// Returns `Err(self)` if the midpoint is outside of `self`
	pub fn split_at(self, mid: usize) -> Result<(Self, Self), Self> {
		if self.start <= mid && mid <= self.end() {
			Ok((
				Span::new(self.start, mid),
				Span::from_range(mid..self.end()),
			))
		} else {
			Err(self)
		}
	}

	/// Like [`split_at`](#method.split_at), but the position is given relative to `self.start`,
	/// i.e. `span.split_at_rel(offs)` ⇔ `span.split_at(span.start() + offs)`
	pub fn split_at_rel(self, offs: usize) -> Result<(Self, Self), Self> {
		if offs <= self.len {
			let mid = self.start + offs;
			Ok((
				Span::new(self.start, mid),
				Span::from_range(mid..self.end()),
			))
		} else {
			Err(self)
		}
	}
}

impl From<Span> for Range<usize> {
	#[inline]
	fn from(span: Span) -> Self {
		span.as_range()
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

impl Index<Span> for str {
	type Output = Self;

	#[inline]
	fn index(&self, index: Span) -> &Self::Output {
		self.index(index.as_range())
	}
}

impl IndexMut<Span> for str {
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
	pub const fn new(span: Span, inner: T) -> Self {
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

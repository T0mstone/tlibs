//! This crate provides the [`BiResult`](struct.BiResult.html) struct.
#![no_std]

use core::iter::{Chain, Map};

/// A `Result`-like struct that always contains a value,
/// and possibly some number of errors.
pub struct BiResult<T, I: IntoIterator>(pub T, pub I);

impl<T, I: IntoIterator> BiResult<T, I> {
    /// Creates a new `BiResult` with `t` as value and
    /// the default value for the errors.
    ///
    /// ## Unintuitiveness
    /// The default value for `I` is not necessarily the designated 'empty' value.
    /// In such cases, how this function behaves may be unexpected.
    pub fn ok(t: T) -> Self
    where
        I: Default,
    {
        Self(t, I::default())
    }

    /// Maps a `BiResult<T, I>` to `BiResult<U, I>` by applying a function to the value, leaving the errors untouched.
    ///
    /// This function can be used to compose the results of two functions.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> BiResult<U, I> {
        BiResult(f(self.0), self.1)
    }

    /// Maps a `BiResult<T, I>` to `BiResult<U, ...>` by applying a function to each error, leaving the value untouched.
    pub fn map_errs<U, F: FnMut(I::Item) -> U>(self, f: F) -> BiResult<T, Map<I::IntoIter, F>> {
        BiResult(self.0, self.1.into_iter().map(f))
    }

    /// Composes two `BiResult`s by applying `f` to unify their values and
    /// by appending the errors from `rhs` to the errors from `self`.
    pub fn and<U, V, F: FnOnce(T, U) -> V, J: IntoIterator<Item = I::Item>>(
        self,
        rhs: BiResult<U, J>,
        f: F,
    ) -> BiResult<V, Chain<I::IntoIter, J::IntoIter>> {
        BiResult(
            f(self.0, rhs.0),
            self.1.into_iter().chain(rhs.1.into_iter()),
        )
    }
}

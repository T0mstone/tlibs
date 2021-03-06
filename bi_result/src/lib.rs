//! This crate provides the [`BiResult`](struct.BiResult.html) struct.
#![no_std]

use core::iter::{once, Chain, FromIterator, Map};

/// A `Result`-like struct that always contains a value,
/// and possibly some number of errors.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct BiResult<T, I: IntoIterator>(pub T, pub I);

impl<T, I: IntoIterator> BiResult<T, I> {
    /// Creates a new `BiResult` with `t` as value and
    /// the default value for the errors.
    ///
    /// ## Unintuitiveness
    /// The default value for `I` is not necessarily the designated 'empty' value.
    /// In such cases, how this function behaves may be unexpected.
    #[inline]
    pub fn ok(t: T) -> Self
    where
        I: Default,
    {
        Self(t, I::default())
    }

    /// Creates a new `BiResult` with `e` as errors and
    /// the default value for the value (`T`).
    #[inline]
    pub fn err(e: I) -> Self
    where
        T: Default,
    {
        Self(T::default(), e)
    }

    /// Maps a `BiResult<T, I>` to `BiResult<U, I>` by applying a function to the value, leaving the errors untouched.
    ///
    /// This function can be used to compose the results of two functions.
    #[inline]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> BiResult<U, I> {
        BiResult(f(self.0), self.1)
    }

    /// Maps a `BiResult<T, I>` to `BiResult<T, V>` by applying a function to the error list, leaving the value untouched.
    #[inline]
    pub fn map_err<V: IntoIterator, F: FnOnce(I) -> V>(self, f: F) -> BiResult<T, V> {
        BiResult(self.0, f(self.1))
    }

    /// Maps a `BiResult<T, I>` to `BiResult<T, ...>` by applying a function to each error, leaving the value untouched.
    #[inline]
    pub fn map_each_err<U, F: FnMut(I::Item) -> U>(self, f: F) -> BiResult<T, Map<I::IntoIter, F>> {
        BiResult(self.0, self.1.into_iter().map(f))
    }

    /// Composes two `BiResult`s by applying `f` to unify their values and
    /// by appending the errors from `rhs` to the errors from `self`.
    pub fn join<U, V, F: FnOnce(T, U) -> V, J: IntoIterator<Item = I::Item>>(
        self,
        rhs: BiResult<U, J>,
        f: F,
    ) -> BiResult<V, Chain<I::IntoIter, J::IntoIter>> {
        BiResult(
            f(self.0, rhs.0),
            self.1.into_iter().chain(rhs.1.into_iter()),
        )
    }

    /// Converts a `Result` to an `Option` by appending any `Err` value
    /// to `self`'s errors
    pub fn consume_err<U, E>(&mut self, r: Result<U, E>) -> Option<U>
    where
        I: Extend<E>,
    {
        r.map_err(|e| {
            self.1.extend(once(e));
        })
        .ok()
    }

    /// Composes the result of applying `f` onto the value of `self`
    /// with the errors from `self`.
    pub fn and_then<U, F: FnOnce(T) -> BiResult<U, J>, J: IntoIterator<Item = I::Item>>(
        self,
        f: F,
    ) -> BiResult<U, Chain<I::IntoIter, J::IntoIter>> {
        let BiResult(u, j) = f(self.0);
        BiResult(u, self.1.into_iter().chain(j.into_iter()))
    }

    /// Extend `target` with all errors and return only the value.
    /// This is similar to the `?` operator on a `Result`
    #[inline]
    pub fn push_errs<V: Extend<I::Item>>(self, target: &mut V) -> T {
        target.extend(self.1);
        self.0
    }

    /// Extend `target` with all errors (applying `Into::into`) and return only the value.
    /// This is similar to the `?` operator on a `Result`
    #[inline]
    pub fn push_errs_with<Item, F: FnMut(I::Item) -> Item, V: Extend<Item>>(
        self,
        target: &mut V,
        f: F,
    ) -> T {
        target.extend(self.1.into_iter().map(f));
        self.0
    }

    /// Extend `target` with all errors (applying `Into::into`) and return only the value.
    /// This is similar to the `?` operator on a `Result`
    #[inline]
    pub fn push_errs_with_into<Item: From<I::Item>, V: Extend<Item>>(self, target: &mut V) -> T {
        target.extend(self.1.into_iter().map(Into::into));
        self.0
    }

    /// returns `self.0`, panics with the provided message if there are any errors
    pub fn expect(self, msg: &str) -> T {
        let n = self.1.into_iter().count();
        if n > 0 {
            panic!("{} ({} errors)", msg, n)
        }
        self.0
    }

    /// Like `expect`, but with a default error message
    pub fn unwrap(self) -> T {
        self.expect("called `BiResult::unwrap()` on a value with errors")
    }
}

impl<VT, VI: IntoIterator, T, I: IntoIterator> FromIterator<BiResult<T, I>> for BiResult<VT, VI>
where
    VT: FromIterator<T> + Default + Extend<T>,
    VI: FromIterator<I> + Default + Extend<I>,
{
    fn from_iter<J: IntoIterator<Item = BiResult<T, I>>>(iter: J) -> Self {
        let (t, i): (VT, VI) = iter
            .into_iter()
            .map(|br: BiResult<T, I>| (br.0, br.1))
            .unzip();
        BiResult(t, i)
    }
}

/// Extensions to the `Result` type
pub trait ResultExt<T, E>: Sized {
    /// Convert a `Result` to a `BiResult`
    fn into_bi_result<I>(self) -> BiResult<T, I>
    where
        T: Default,
        I: Default + Extend<E> + IntoIterator<Item = E>;

    /// push the error in an `Err(_)` onto a collection, returning an `Option`
    fn push_error<I>(self, errs: &mut I) -> Option<T>
    where
        I: Extend<E>;

    /// push the error in an `Err(_)` onto a collection, returning the value from an `Ok(_)` or `T::default()` otherwise
    fn push_error_or_default<I>(self, errs: &mut I) -> T
    where
        T: Default,
        I: Extend<E>,
    {
        self.push_error(errs).unwrap_or_default()
    }
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn into_bi_result<I>(self) -> BiResult<T, I>
    where
        T: Default,
        I: Default + Extend<E> + IntoIterator<Item = E>,
    {
        match self {
            Ok(t) => BiResult::ok(t),
            Err(e) => BiResult::err({
                let mut tmp = I::default();
                tmp.extend(once(e));
                tmp
            }),
        }
    }

    fn push_error<I>(self, errs: &mut I) -> Option<T>
    where
        I: Extend<E>,
    {
        match self {
            Ok(t) => Some(t),
            Err(e) => {
                errs.extend(once(e));
                None
            }
        }
    }
}

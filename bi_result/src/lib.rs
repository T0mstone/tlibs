#![no_std]

use core::iter::{Chain, Map};

pub struct BiResult<T, I: IntoIterator>(pub T, pub I);

impl<T, I: IntoIterator> BiResult<T, I> {
    pub fn ok_default(t: T) -> Self
    where
        I: Default,
    {
        Self(t, I::default())
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> BiResult<U, I> {
        BiResult(f(self.0), self.1)
    }

    pub fn map_errs<U, F: FnMut(I::Item) -> U>(self, f: F) -> BiResult<T, Map<I::IntoIter, F>> {
        BiResult(self.0, self.1.into_iter().map(f))
    }

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

impl<T, I: IntoIterator> BiResult<T, Option<I>> {
    pub fn ok(t: T) -> Self {
        Self(t, None)
    }
}

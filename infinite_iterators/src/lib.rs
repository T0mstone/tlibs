#![cfg_attr(not(test), no_std)]
//! `infinite_iterators`: Iterators that never end

use self::iters::*;
use core::iter::FusedIterator;
use core::iter::{Cycle, Repeat};

// todo: documentation

pub mod iters {
    use super::{FusedIterator, InfiniteIterator};

    pub struct Chain<A, B> {
        pub(super) left: Option<A>,
        pub(super) right: B,
    }

    impl<T, A: Iterator<Item = T> + FusedIterator, B: InfiniteIterator<Item = T>> InfiniteIterator
        for Chain<A, B>
    {
        type Item = T;

        fn next(&mut self) -> Self::Item {
            match self.left.as_mut().map(|iter| iter.next()) {
                Some(Some(t)) => return t,
                Some(None) => self.left = None,
                None => (),
            }
            self.right.next()
        }
    }

    pub struct StepBy<I> {
        pub(super) iter: I,
        pub(super) step: usize,
        pub(super) first_take: bool,
    }

    impl<I: InfiniteIterator> StepBy<I> {
        pub(super) fn new(iter: I, step: usize) -> Self {
            assert_ne!(step, 0);
            Self {
                iter,
                step: step - 1,
                first_take: true,
            }
        }
    }

    impl<I: InfiniteIterator> InfiniteIterator for StepBy<I> {
        type Item = I::Item;

        fn next(&mut self) -> I::Item {
            if self.first_take {
                self.first_take = false;
                self.iter.next()
            } else {
                self.iter.nth(self.step)
            }
        }
    }

    pub struct Take<I> {
        pub(super) iter: I,
        pub(super) n: usize,
    }

    impl<I: InfiniteIterator> Iterator for Take<I> {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            if self.n == 0 {
                None
            } else {
                self.n -= 1;
                Option::Some(self.iter.next())
            }
        }
    }

    /// A struct that converts between `Iterator`
    /// and `InfiniteIterator` both ways
    pub struct Inf<I> {
        pub(super) iter: I,
    }

    impl<I: InfiniteIterator> Iterator for Inf<I> {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            Some(self.iter.next())
        }
    }

    impl<I: Iterator> InfiniteIterator for Inf<I> {
        type Item = I::Item;

        fn next(&mut self) -> Self::Item {
            self.iter.next().expect("ITerator inside Inf<I> ended")
        }
    }
}

/// Promote an iterator to an infinite iterator
pub trait PromiseInfinite: Iterator + Sized {
    fn promise_infinite(self) -> Inf<Self> {
        Inf { iter: self }
    }
}

impl<I: Iterator> PromiseInfinite for I {}

pub trait ChainInfinite: Iterator + FusedIterator + Sized {
    fn chain_infinite<I: InfiniteIterator>(self, right: I) -> Chain<Self, I> {
        Chain {
            left: Some(self),
            right,
        }
    }
}

impl<I: Iterator + FusedIterator> ChainInfinite for I {}

/// An iterator that never ends
pub trait InfiniteIterator {
    type Item;

    fn next(&mut self) -> Self::Item;

    fn nth(&mut self, mut n: usize) -> Self::Item {
        loop {
            let x = self.next();
            if n == 0 {
                return x;
            }
            n -= 1;
        }
    }

    fn step_by(self, step: usize) -> StepBy<Self>
    where
        Self: Sized,
    {
        StepBy::new(self, step)
    }

    // todo:
    //  - zip
    //  - map
    //  - for_each
    //  - filter
    //  - filter_map
    //  - enumerate
    //  - peekable
    //  - skip_while
    //  - take_while
    //  - skip

    fn take(self, n: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take { iter: self, n }
    }

    // todo:
    //  - flat_map
    //  - flatten
    //  - inspect
    //  - by_ref
    //  - copied
    //  - cloned

    fn iterator(self) -> Inf<Self>
    where
        Self: Sized,
    {
        Inf { iter: self }
    }
}

impl<I> InfiniteIterator for Cycle<I>
where
    Cycle<I>: Iterator,
{
    type Item = <Cycle<I> as Iterator>::Item;

    fn next(&mut self) -> Self::Item {
        Iterator::next(self).unwrap()
    }
}

impl<A: Clone> InfiniteIterator for Repeat<A> {
    type Item = A;

    fn next(&mut self) -> Self::Item {
        Iterator::next(self).unwrap()
    }
}

pub struct FromFn<F>(F);

pub fn from_fn<T, F: FnMut() -> T>(f: F) -> FromFn<F> {
    FromFn(f)
}

impl<T, F: FnMut() -> T> InfiniteIterator for FromFn<F> {
    type Item = T;

    fn next(&mut self) -> Self::Item {
        (self.0)()
    }
}

pub struct Successors<T, F> {
    t: T,
    f: F,
}

pub fn successors<T, F: FnMut(&T) -> T>(first: T, succ: F) -> Successors<T, F> {
    Successors { t: first, f: succ }
}

impl<T, F: FnMut(&T) -> T> InfiniteIterator for Successors<T, F> {
    type Item = T;

    fn next(&mut self) -> Self::Item {
        let next = (self.f)(&self.t);
        core::mem::replace(&mut self.t, next)
    }
}

pub struct Decaying<T>(T);

impl<T: Default> InfiniteIterator for Decaying<T> {
    type Item = T;
    fn next(&mut self) -> T {
        core::mem::take(&mut self.0)
    }
}

pub fn decaying<T: Default>(t: T) -> Decaying<T> {
    Decaying(t)
}

#[inline]
pub fn true_once() -> impl InfiniteIterator<Item = bool> {
    decaying(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::{once, repeat};

    fn inf_next<I: InfiniteIterator>(i: &mut I) -> I::Item {
        i.next()
    }

    #[test]
    fn promote() {
        let mut x = std::iter::from_fn(|| Some(0)).promise_infinite();
        assert_eq!(x.next(), 0);
        assert_eq!(x.next(), 0);
        assert_eq!(x.next(), 0);
    }

    #[test]
    fn iterate() {
        let mut x = vec![1, 2, 3].into_iter().cycle().iterator();
        assert_eq!(inf_next(&mut x), 1);
        assert_eq!(inf_next(&mut x), 2);
        assert_eq!(inf_next(&mut x), 3);
        assert_eq!(inf_next(&mut x), 1);
    }

    #[test]
    fn decaying_bool() {
        let mut x = true_once();
        assert_eq!(x.next(), true);
        assert_eq!(x.next(), false);
        assert_eq!(x.next(), false);
    }

    #[test]
    fn chain_int() {
        let mut x = once(1).chain_infinite(repeat(0));
        assert_eq!(x.next(), 1);
        assert_eq!(x.next(), 0);
        assert_eq!(x.next(), 0);
    }
}

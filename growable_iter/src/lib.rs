//! This crate provides the [`GrowableIterator`](struct.GrowableIterator.html) struct.

use core::iter::FusedIterator;
use std::convert::Infallible;
use vec_like::*;

/// An iterator that can grow to both sides after creation
///
/// # Generic parameters
/// `I` is the inner iterator,
/// `F` is the type that will be used as storage for growth to the front and
/// `B` is the type that will be used as storage for growth to the back
pub struct GrowableIterator<I: Iterator, F, B> {
    front: F,
    iter: I,
    back: B,
}

impl<T, I: Iterator<Item = T>, F: PushFront<T>, B> PushFront<T> for GrowableIterator<I, F, B> {
    type Err = F::Err;

    fn push_front(&mut self, t: T) -> Result<(), F::Err> {
        self.front.push_front(t)
    }
}

impl<T, I: Iterator<Item = T>, F, B: PushBack<T>> PushBack<T> for GrowableIterator<I, F, B> {
    type Err = B::Err;

    fn push_back(&mut self, t: T) -> Result<(), B::Err> {
        self.back.push_back(t)
    }
}

impl<T, I: Iterator<Item = T>, F: PushBack<T, Err = Infallible> + PeekFront<T>, B> PeekFront<T>
    for GrowableIterator<I, F, B>
{
    fn peek_front(&mut self) -> Option<&T> {
        if self.front.peek_front().is_none() {
            let item = self.iter.next()?;
            let _ = self.front.push_back(item);
        }
        self.front.peek_front()
    }
}

impl<T, I, F, B> PeekBack<T> for GrowableIterator<I, F, B>
where
    I: DoubleEndedIterator<Item = T> + FusedIterator<Item = T>,
    B: PushFront<T, Err = Infallible> + PeekBack<T>,
{
    fn peek_back(&mut self) -> Option<&T> {
        if self.back.peek_back().is_none() {
            let item = self.iter.next_back()?;
            let _ = self.back.push_front(item);
        }
        self.back.peek_back()
    }
}

/// A trait to create a `GrowableIterator` from a regular `FusedIterator`
pub trait Growable: FusedIterator + Sized {
    /// creates a `GrowableIterator` from `self`, `front` and `back`
    fn growable<F, B>(self, front: F, back: B) -> GrowableIterator<Self, F, B> {
        GrowableIterator {
            front,
            iter: self,
            back,
        }
    }
}

impl<I: FusedIterator> Growable for I {}

impl<T, I, F, B> Iterator for GrowableIterator<I, F, B>
where
    I: FusedIterator<Item = T>,
    F: IntoIterator<Item = T> + PopFront<T>,
    B: IntoIterator<Item = T> + PopFront<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.front
            .pop_front()
            .or_else(|| self.iter.next())
            .or_else(|| self.back.pop_front())
    }
}

impl<T, I, F, B> FusedIterator for GrowableIterator<I, F, B>
where
    I: FusedIterator<Item = T>,
    F: IntoIterator<Item = T> + PopFront<T>,
    B: IntoIterator<Item = T> + PopFront<T>,
{
}

impl<T, I, F, B> DoubleEndedIterator for GrowableIterator<I, F, B>
where
    I: FusedIterator<Item = T> + DoubleEndedIterator<Item = T>,
    F: IntoIterator<Item = T> + PopFront<T> + PopBack<T>,
    B: IntoIterator<Item = T> + PopFront<T> + PopBack<T>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.back
            .pop_back()
            .or_else(|| self.iter.next_back())
            .or_else(|| self.front.pop_back())
    }
}

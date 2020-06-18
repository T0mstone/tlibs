#![cfg_attr(not(feature = "use-std"), no_std)]
use core::convert::Infallible;

pub trait PushFront<T> {
    type Err;

    fn push_front(&mut self, t: T) -> Result<(), Self::Err>;
}

pub trait PushBack<T> {
    type Err;

    fn push_back(&mut self, t: T) -> Result<(), Self::Err>;
}

pub trait PopFront<T> {
    fn pop_front(&mut self) -> Option<T>;
}

pub trait PopBack<T> {
    fn pop_back(&mut self) -> Option<T>;
}

pub trait PeekFront<T: ?Sized> {
    fn peek_front(&mut self) -> Option<&T>;
}

pub trait PeekBack<T: ?Sized> {
    fn peek_back(&mut self) -> Option<&T>;
}

#[cfg(feature = "use-std")]
mod std_impls {
    use super::*;
    use std::collections::VecDeque;

    // section: impl Vec

    impl<T> PushFront<T> for Vec<T> {
        type Err = Infallible;

        fn push_front(&mut self, t: T) -> Result<(), Self::Err> {
            self.insert(0, t);
            Ok(())
        }
    }

    impl<T> PushBack<T> for Vec<T> {
        type Err = Infallible;

        fn push_back(&mut self, t: T) -> Result<(), Self::Err> {
            self.push(t);
            Ok(())
        }
    }

    impl<T> PopFront<T> for Vec<T> {
        fn pop_front(&mut self) -> Option<T> {
            if self.is_empty() {
                None
            } else {
                Some(self.remove(0))
            }
        }
    }

    impl<T> PopBack<T> for Vec<T> {
        fn pop_back(&mut self) -> Option<T> {
            self.pop()
        }
    }

    impl<T> PeekBack<T> for Vec<T> {
        fn peek_back(&mut self) -> Option<&T> {
            self.last()
        }
    }

    impl<T> PeekFront<T> for Vec<T> {
        fn peek_front(&mut self) -> Option<&T> {
            self.first()
        }
    }

    // section: impl VecDeque

    impl<T> PushFront<T> for VecDeque<T> {
        type Err = Infallible;

        fn push_front(&mut self, t: T) -> Result<(), Self::Err> {
            self.push_front(t);
            Ok(())
        }
    }

    impl<T> PushBack<T> for VecDeque<T> {
        type Err = Infallible;

        fn push_back(&mut self, t: T) -> Result<(), Self::Err> {
            self.push_back(t);
            Ok(())
        }
    }

    impl<T> PopFront<T> for VecDeque<T> {
        fn pop_front(&mut self) -> Option<T> {
            self.pop_front()
        }
    }

    impl<T> PopBack<T> for VecDeque<T> {
        fn pop_back(&mut self) -> Option<T> {
            self.pop_back()
        }
    }

    impl<T> PeekBack<T> for VecDeque<T> {
        fn peek_back(&mut self) -> Option<&T> {
            self.back()
        }
    }

    impl<T> PeekFront<T> for VecDeque<T> {
        fn peek_front(&mut self) -> Option<&T> {
            self.front()
        }
    }
}

// section: impl Option

impl<T> PushFront<T> for Option<T> {
    type Err = T;

    fn push_front(&mut self, t: T) -> Result<(), Self::Err> {
        match self {
            Some(_) => Err(t),
            None => {
                *self = Some(t);
                Ok(())
            }
        }
    }
}

impl<T> PushBack<T> for Option<T> {
    type Err = T;

    fn push_back(&mut self, t: T) -> Result<(), Self::Err> {
        match self {
            Some(_) => Err(t),
            None => {
                *self = Some(t);
                Ok(())
            }
        }
    }
}

impl<T> PopFront<T> for Option<T> {
    fn pop_front(&mut self) -> Option<T> {
        self.take()
    }
}

impl<T> PopBack<T> for Option<T> {
    fn pop_back(&mut self) -> Option<T> {
        self.take()
    }
}

impl<T> PeekBack<T> for Option<T> {
    fn peek_back(&mut self) -> Option<&T> {
        self.as_ref()
    }
}

impl<T> PeekFront<T> for Option<T> {
    fn peek_front(&mut self) -> Option<&T> {
        self.as_ref()
    }
}

// section: impl ()

impl<T> PushFront<T> for () {
    type Err = T;

    fn push_front(&mut self, t: T) -> Result<(), Self::Err> {
        Err(t)
    }
}

impl<T> PushBack<T> for () {
    type Err = T;

    fn push_back(&mut self, t: T) -> Result<(), Self::Err> {
        Err(t)
    }
}

impl<T> PopFront<T> for () {
    fn pop_front(&mut self) -> Option<T> {
        None
    }
}

impl<T> PopBack<T> for () {
    fn pop_back(&mut self) -> Option<T> {
        None
    }
}

impl<T> PeekBack<T> for () {
    fn peek_back(&mut self) -> Option<&T> {
        None
    }
}

impl<T> PeekFront<T> for () {
    fn peek_front(&mut self) -> Option<&T> {
        None
    }
}

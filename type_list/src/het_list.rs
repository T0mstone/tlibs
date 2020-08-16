use self::private::Sealed;

mod private {

    pub trait Sealed {}

    impl Sealed for super::Nil {}
    impl<T, Ts: super::HetList> Sealed for super::Cons<T, Ts> {}
}

/// The core trait that any heterogeneous list implements
pub trait HetList: Sealed + Sized {
    /// The type of the first element
    type Head;
    /// The type of the rest of the list
    type Tail: HetList;
    /// The length of the list
    const LEN: usize;

    /// The first element
    fn head(&self) -> &Self::Head;
    /// The rest of the list
    fn tail(&self) -> &Self::Tail;
    /// The first element mutable
    fn head_mut(&mut self) -> &mut Self::Head;
    /// The rest of the list mutable
    fn tail_mut(&mut self) -> &mut Self::Tail;
    /// The first element, consuming `self`
    fn into_head(self) -> Self::Head;
    /// The rest of the list, consuming `self`
    fn into_tail(self) -> Self::Tail;
    /// The first element and the rest of the list, consuming `self`
    fn into_head_tail(self) -> (Self::Head, Self::Tail);
}

/// The empty heterogeneous list
pub struct Nil;

impl HetList for Nil {
    type Head = ();
    type Tail = Self;
    const LEN: usize = 0;

    fn head(&self) -> &Self::Head {
        &()
    }
    fn tail(&self) -> &Self::Tail {
        &Self
    }
    fn head_mut(&mut self) -> &mut Self::Head {
        static mut UNIT: () = ();
        // SAFETY: this can't be modified because it is a ZST, thus it's safe
        unsafe { &mut UNIT }
    }
    fn tail_mut(&mut self) -> &mut Self::Tail {
        self
    }
    fn into_head(self) -> Self::Head {
        ()
    }
    fn into_tail(self) -> Self::Tail {
        Self
    }
    fn into_head_tail(self) -> (Self::Head, Self::Tail) {
        ((), Self)
    }
}

/// The heterogeneous list with a head of type `T` and a tail of type `Ts`
pub struct Cons<T, Ts: HetList> {
    head: T,
    tail: Ts,
}

/// a helper function to ease the creation of new heterogeneous lists
#[inline]
pub fn cons<T, Ts: HetList>(head: T, tail: Ts) -> Cons<T, Ts> {
    Cons { head, tail }
}

impl<T, Ts: HetList> HetList for Cons<T, Ts> {
    type Head = T;
    type Tail = Ts;
    const LEN: usize = Ts::LEN + 1;

    fn head(&self) -> &Self::Head {
        &self.head
    }
    fn tail(&self) -> &Self::Tail {
        &self.tail
    }
    fn head_mut(&mut self) -> &mut Self::Head {
        &mut self.head
    }
    fn tail_mut(&mut self) -> &mut Self::Tail {
        &mut self.tail
    }
    fn into_head(self) -> Self::Head {
        self.head
    }
    fn into_tail(self) -> Self::Tail {
        self.tail
    }
    fn into_head_tail(self) -> (Self::Head, Self::Tail) {
        (self.head, self.tail)
    }
}

/// A function-trait.
/// Its `Output` type is the list that represents `[..., T]` where `...` are the types of the elements in `Self`
/// and its `append` function provides a means to create a value of that type
pub trait AppendItem<T>: HetList {
    /// The list that represents `[..., T]` where `...` are all elements in `Self`
    type Output: HetList;
    /// appends an element to the end of the list
    fn append(self, t: T) -> Self::Output;
}

impl<T> AppendItem<T> for Nil {
    type Output = Cons<T, Nil>;
    fn append(self, t: T) -> Self::Output {
        cons(t, Nil)
    }
}

impl<U, T, Ts: AppendItem<U>> AppendItem<U> for Cons<T, Ts> {
    type Output = Cons<T, Ts::Output>;

    fn append(self, u: U) -> Self::Output {
        cons(self.head, self.tail.append(u))
    }
}

/// A macro that creates a heterogeneous list. The syntax is like `vec!`
#[macro_export]
macro_rules! het_list {
    () => {
        $crate::het_list::Nil
    };
    ($x:expr $(, $xs:expr) *$(,)?) => {
    	$crate::het_list::cons($x, $crate::het_list::het_list!($($xs),*))
    };
}

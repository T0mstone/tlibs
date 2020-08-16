use self::private::Sealed;
use std::marker::PhantomData;

mod private {
    pub trait Sealed {}

    impl Sealed for super::Nil {}
    impl<T: ?Sized, Ts: super::TypeList> Sealed for super::Cons<T, Ts> {}
}

/// The core trait that any type list implements
pub trait TypeList: Sealed {
    /// The first item of the list
    type Head;
    /// The rest of the list
    type Tail: TypeList;
    /// The length of the list
    const LEN: usize;
}
/// The empty type list
pub enum Nil {}
impl TypeList for Nil {
    type Head = ();
    type Tail = Self;
    const LEN: usize = 0;
}

/// `Cons<T, ...>` represents the list `[T, ...]` where `...` are all elements in `Ts`
pub struct Cons<T: ?Sized, Ts: TypeList> {
    #[allow(missing_docs)]
    _head: PhantomData<T>,
    /// Because this is required by value, you can never construct a value of type `Cons`
    _tail: Ts,
}
impl<T, Ts: TypeList> TypeList for Cons<T, Ts> {
    type Head = T;
    type Tail = Ts;
    const LEN: usize = Ts::LEN + 1;
}

/// A function-trait. Its `Output` type is the list that represents `[..., T]` where `...` are all elements in `Self`
pub trait AppendType<T>: TypeList {
    /// The list that represents `[..., T]` where `...` are all elements in `Self`
    type Output: TypeList;
}
impl<T> AppendType<T> for Nil {
    type Output = Cons<T, Nil>;
}
impl<T, U, Us: AppendType<T>> AppendType<T> for Cons<U, Us> {
    type Output = Cons<U, <Us as AppendType<T>>::Output>;
}

/// An alias to append a type to the end of a type list
pub type Append<Ts, T> = <Ts as AppendType<T>>::Output;

/// A macro that creates a type list. The syntax is like `vec!`, but with types
#[macro_export]
macro_rules! type_list {
    () => {
        $crate::type_list::Nil
    };
    ($t:ty $(, $ts:ty)* $(,)?) => {
        $crate::type_list::Cons<$t, $crate::type_list::type_list![$($ts),*]>
    };
}

use std::marker::PhantomData;

pub trait TypeList {
    type Head;
    type Tail: TypeList;
    const LEN: usize;
}
pub enum Nil {}
impl TypeList for Nil {
    type Head = ();
    type Tail = Self;
    const LEN: usize = 0;
}
pub struct Cons<T: ?Sized, Ts: TypeList + ?Sized> {
    _marker1: PhantomData<T>,
    _marker2: PhantomData<Ts>,
}
impl<T, Ts: TypeList> TypeList for Cons<T, Ts> {
    type Head = T;
    type Tail = Ts;
    const LEN: usize = Ts::LEN + 1;
}

pub trait AppendType<T>: TypeList {
    type Output: TypeList;
}
impl<T> AppendType<T> for Nil {
    type Output = Cons<T, Nil>;
}
impl<T, U, Us: AppendType<T>> AppendType<T> for Cons<U, Us> {
    type Output = Cons<U, <Us as AppendType<T>>::Output>;
}

pub type Append<Ts, T> = <Ts as AppendType<T>>::Output;

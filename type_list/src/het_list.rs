pub trait HetList {
    type Head;
    type Tail: HetList;
    const LEN: usize;
}

pub struct Nil;

impl HetList for Nil {
    type Head = ();
    type Tail = Self;
    const LEN: usize = 0;
}

pub struct Cons<T, Ts: HetList> {
    head: T,
    tail: Ts,
}

#[inline]
pub fn cons<T, Ts: HetList>(head: T, tail: Ts) -> Cons<T, Ts> {
    Cons { head, tail }
}

impl<T, Ts: HetList> HetList for Cons<T, Ts> {
    type Head = T;
    type Tail = Ts;
    const LEN: usize = Ts::LEN + 1;
}

pub trait AppendItem<T>: HetList {
    type Output: HetList;
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

pub type Append<Ts, T> = <Ts as AppendItem<T>>::Output;

#[macro_export]
macro_rules! het_list {
    () => {
        $crate::het_list::Nil
    };
    ($x:expr $(, $xs:expr)*$(,)?) => {
    	$crate::het_list::cons($x, het_list!($($xs),*))
    };
}

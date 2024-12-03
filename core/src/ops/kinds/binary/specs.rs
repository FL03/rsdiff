/*
    Appellation: specs <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub type BoxedBinOp<A, B = A, C = A> = Box<dyn BinOp<A, B, Output = C>>;

pub trait BinOp<A, B = A> {
    type Output;

    fn eval(&self, lhs: A, rhs: B) -> Self::Output;
}

pub trait BinaryAssignOp<A, B = A> {
    fn eval(&self, lhs: &mut A, rhs: B);
}

impl<S, A, B, C> BinOp<A, B> for S
where
    S: Fn(A, B) -> C,
{
    type Output = C;

    fn eval(&self, lhs: A, rhs: B) -> Self::Output {
        self(lhs, rhs)
    }
}

impl<A, B, C> BinOp<A, B> for Box<dyn BinOp<A, B, Output = C>> {
    type Output = C;

    fn eval(&self, lhs: A, rhs: B) -> Self::Output {
        self.as_ref().eval(lhs, rhs)
    }
}

impl<A, B> BinaryAssignOp<A, B> for Box<dyn BinaryAssignOp<A, B>> {
    fn eval(&self, lhs: &mut A, rhs: B) {
        self.as_ref().eval(lhs, rhs)
    }
}

pub trait Log<T> {
    type Output;

    fn log(self, base: T) -> Self::Output;
}

macro_rules! impl_log {
    ($($t:ty),*) => {
        $(
            impl_log!(@impl $t);
        )*
    };
    ($($call:ident<$out:ty>($t:ty)),*) => {
        $(
            impl_log!(@impl $call<$out>($t));
        )*
    };
    ($call:ident<$out:ty>: $($t:ty),*) => {
        $(
            impl_log!(@impl $call<$out>($t));
        )*
    };
    (@impl $t:ty) => {
        impl Log<$t> for $t {
            type Output = $t;

            fn log(self, base: $t) -> Self::Output {
                self.log(base)
            }
        }
    };
    (@impl $call:ident<$out:ty>($t:ty)) => {
        impl Log<$t> for $t {
            type Output = $out;

            fn log(self, base: $t) -> Self::Output {
                self.$call(base)
            }
        }
    };
}

impl_log!(f32, f64);

impl_log!(ilog<u32>: i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

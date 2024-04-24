/*
    Appellation: specs <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub type BoxedBinOp<A, B = A, C = A> = Box<dyn BinOp<A, B, Output = C>>;

pub trait BinaryOperand<A, B> {
    type Args: crate::ops::Params<Pattern = (A, B)>;
    type Output;
}

pub trait BinOp<A, B = A> {
    type Output;

    fn eval(&self, lhs: A, rhs: B) -> Self::Output;
}

pub trait BinaryAssignOp<A, B = A> {
    fn eval(&self, lhs: A, rhs: B);
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
    fn eval(&self, lhs: A, rhs: B) {
        self.as_ref().eval(lhs, rhs)
    }
}

pub trait Logarithm<T> {
    type Output;

    fn log(self, base: T) -> Self::Output;
}

macro_rules! impl_log {
    ($t:ty) => {
        impl Logarithm<$t> for $t {
            type Output = $t;

            fn log(self, base: $t) -> Self::Output {
                self.log(base)
            }
        }
    };
    (other $t:ty => $out:ty; $method:ident) => {
        impl Logarithm<$t> for $t {
            type Output = $out;

            fn log(self, base: $t) -> Self::Output {
                self.$method(base)
            }
        }
    };
    (all [$($t:ty),*]) => {
        $(
            impl_log!($t);
        )*
    };
}

impl_log!(all [f32, f64]);

impl_log!(other i8 => u32; ilog);
impl_log!(other i16 => u32; ilog);
impl_log!(other i32 => u32; ilog);
impl_log!(other i64 => u32; ilog);
impl_log!(other i128 => u32; ilog);
impl_log!(other isize => u32; ilog);
impl_log!(other u8 => u32; ilog);
impl_log!(other u16 => u32; ilog);
impl_log!(other u32 => u32; ilog);
impl_log!(other u64 => u32; ilog);
impl_log!(other u128 => u32; ilog);
impl_log!(other usize => u32; ilog);

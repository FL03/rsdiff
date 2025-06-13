/*
    Appellation: eval <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait EvalOnce {
    type Output;

    fn eval_once(self) -> Self::Output;
}

pub trait EvalMut: EvalOnce {
    fn eval_mut(&mut self) -> Self::Output;
}

pub trait Eval: EvalMut {
    fn eval(&self) -> Self::Output;
}

macro_rules! impl_eval {

    ($($s:ty),*) => {
        $(
            impl_eval!(@impl $s);
        )*
    };

    (@impl $s:ty) => {
        impl EvalOnce for $s {
            type Output = $s;

            fn eval_once(self) -> Self::Output {
                self
            }
        }

        impl EvalMut for $s {
            fn eval_mut(&mut self) -> Self::Output {
                *self
            }
        }

        impl Eval for $s {
            fn eval(&self) -> Self::Output {
                *self
            }
        }
    };
    (@impl $t:ident.$call:ident<$($res:tt)*>) => {
        impl EvalOnce for $t {
            type Output = $($res)*;

            fn eval_once(self) -> Self::Output {
                self.$call()
            }
        }

        impl EvalMut for $t {
            fn eval_mut(&mut self) -> Self::Output {
                *self.$call()
            }
        }

        impl Eval for $t {
            fn eval(&self) -> Self::Output {
                *self.$call()
            }
        }
    };
    ($ty:ty => $e:expr) => {
        impl EvalOnce for $ty {
            type Output = $ty;

            fn eval_once(self) -> Self::Output {
                $e(self)
            }
        }

        impl EvalMut for $ty {
            fn eval_mut(&mut self) -> Self::Output {
                $e(*self)
            }
        }

        impl Eval for $ty {
            fn eval(&self) -> Self::Output {
                $e(*self)
            }
        }
    };
}

impl_eval!(
    f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

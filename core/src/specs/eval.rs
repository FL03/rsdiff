/*
    Appellation: eval <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait EvaluateOnce {
    type Output;

    fn eval_once(self) -> Self::Output;
}

pub trait EvaluateMut: EvaluateOnce {
    fn eval_mut(&mut self) -> Self::Output;
}

pub trait Evaluate: EvaluateMut {
    fn eval(&self) -> Self::Output;
}

macro_rules! impl_evaluate {
    ($($ty:ty),*) => {
        $(
            impl_evaluate!(@primitive $ty);
        )*
    };
    (@primitive $ty:ty) => {
        impl EvaluateOnce for $ty {
            type Output = $ty;

            fn eval_once(self) -> Self::Output {
                self
            }
        }

        impl EvaluateMut for $ty {
            fn eval_mut(&mut self) -> Self::Output {
                *self
            }
        }

        impl Evaluate for $ty {
            fn eval(&self) -> Self::Output {
                *self
            }
        }
    };
    ($ty:ty, $e:expr) => {
        impl EvaluateOnce for $ty {
            type Output = $ty;

            fn eval_once(self) -> Self::Output {
                $e(self)
            }
        }

        impl EvaluateMut for $ty {
            fn eval_mut(&mut self) -> Self::Output {
                $e(*self)
            }
        }

        impl Evaluate for $ty {
            fn eval(&self) -> Self::Output {
                $e(*self)
            }
        }
    };
}

impl_evaluate!(f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

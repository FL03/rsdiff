/*
   Appellation: unary <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Unary Operations
//!
//!
pub use self::{kinds::*, operator::*, specs::*};

pub(crate) mod kinds;
pub(crate) mod operator;
pub(crate) mod specs;

use num::{Complex, Num};

pub trait UnaryOperation {
    type Output;

    fn unary(self, expr: UnaryOp) -> Self::Output;
}

///
pub trait Conjugate {
    type Complex;
    type Real;

    fn conj(&self) -> Self::Complex;
}

macro_rules! impl_conj {
    ($t:ty) => {
        impl Conjugate for $t {
            type Complex = Complex<Self>;
            type Real = Self;

            fn conj(&self) -> Self::Complex {
                Complex::new(*self, <$t>::default())
            }
        }
    };
    ($($t:ty),*) => {
        $(
            impl_conj!($t);
        )*
    };
}

impl<T> Conjugate for Complex<T>
where
    T: Clone + Num + std::ops::Neg<Output = T>,
{
    type Complex = Self;
    type Real = T;

    fn conj(&self) -> Self::Complex {
        Complex::conj(self)
    }
}

impl_conj!(i8, i16, i32, i64, i128, isize);
impl_conj!(f32, f64);

#[cfg(test)]
mod tests {}

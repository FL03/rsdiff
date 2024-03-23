/*
    Appellation: specs <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod ndtensor;
pub mod scalar;

use num::{Complex, Num};

pub trait Affine<T> {
    type Output;

    fn affine(&self, mul: &T, add: &T) -> Self::Output;
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

impl_conj!(u8, u16, u32, u64, u128, usize);
impl_conj!(i8, i16, i32, i64, i128, isize);
impl_conj!(f32, f64);

pub(crate) mod prelude {
    pub use super::ndtensor::*;
    pub use super::scalar::*;
    pub use super::Affine;
}

#[cfg(test)]
mod tests {
    // use super::*;

    macro_rules! Scalar {
        (complex) => {
            Scalar!(cf64)
        };
        (float) => {
            Scalar!(f64)
        };
        (cf64) => {
            Complex<f64>
        };
        (cf32) => {
            Complex<f32>
        };
        (f64) => {
            f64
        };
        (f32) => {
            f32
        };

    }

    #[test]
    fn test_scalar() {
        let a: Scalar!(f64);
        a = 3.0;
        assert_eq!(a, 3_f64);
    }
}

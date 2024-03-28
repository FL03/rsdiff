/*
    Appellation: specs <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod ndtensor;
pub mod scalar;

pub trait Affine<T> {
    type Output;

    fn affine(&self, mul: &T, add: &T) -> Self::Output;
}

pub trait Vstack<T> {
    type Output;

    fn vstack(&self, other: &T) -> Self::Output;
}

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

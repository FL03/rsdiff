/*
    Appellation: specs <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{affine::*, ndtensor::*, reshape::*, scalar::*};

pub(crate) mod affine;
pub(crate) mod ndtensor;
pub(crate) mod reshape;
pub(crate) mod scalar;

pub trait Hstack<T> {
    type Output;

    fn hstack(&self, other: &T) -> Self::Output;
}

pub trait Vstack<T> {
    type Output;

    fn vstack(&self, other: &T) -> Self::Output;
}

pub(crate) mod prelude {
    pub use super::affine::*;
    pub use super::ndtensor::*;
    pub use super::reshape::*;
    pub use super::scalar::*;
}

#[cfg(test)]
mod tests {
    use super::scalar::Scalar;
    use num::Complex;

    #[test]
    fn test_scalar() {
        let a = 3f64;
        let b = Complex::new(4f64, 0f64);

        assert_eq!(Scalar::sqr(a), 9f64);
        assert_eq!(Scalar::sqrt(b), 2f64.into());
    }
}

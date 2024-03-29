/*
    Appellation: specs <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{affine::*, ndtensor::*, scalar::*};

pub(crate) mod affine;
pub(crate) mod ndtensor;
pub(crate) mod scalar;

pub trait Hstack<T> {
    type Output;

    fn hstack(&self, other: &T) -> Self::Output;
}

pub trait Vstack<T> {
    type Output;

    fn vstack(&self, other: &T) -> Self::Output;
}

pub trait Swap {
    type Key;

    fn swap(&mut self, swap: Self::Key, with: Self::Key);
}

impl<T> Swap for [T] {
    type Key = usize;

    fn swap(&mut self, swap: Self::Key, with: Self::Key) {
        self.swap(swap, with);
    }
}

pub(crate) mod prelude {
    pub use super::ndtensor::*;
    pub use super::scalar::*;
    pub use super::Affine;
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

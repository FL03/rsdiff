/*
    Appellation: traits <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{eval::*, propagate::*, scalar::Scalar};

pub mod eval;
pub mod propagate;
pub mod scalar;

pub trait AsSlice<T> {
    fn as_slice(&self) -> &[T];
}

pub trait AsSliceMut<T> {
    fn as_mut_slice(&mut self) -> &mut [T];
}

impl<S, T> AsSlice<T> for S
where
    S: AsRef<[T]>,
{
    fn as_slice(&self) -> &[T] {
        self.as_ref()
    }
}

impl<S, T> AsSliceMut<T> for S
where
    S: AsMut<[T]>,
{
    fn as_mut_slice(&mut self) -> &mut [T] {
        self.as_mut()
    }
}

pub trait Gradient<T> {
    type Gradient;

    fn grad(&self, args: T) -> Self::Gradient;
}

pub(crate) mod prelude {
    pub use super::eval::*;
    pub use super::propagate::*;
    pub use super::scalar::Scalar;
    pub use super::{AsSlice, AsSliceMut, Gradient};
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

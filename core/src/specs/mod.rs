/*
    Appellation: specs <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub use self::{eval::*, gradient::*, prop::*, scalar::Scalar, store::*};

pub(crate) mod eval;
pub(crate) mod gradient;
pub(crate) mod prop;
pub(crate) mod scalar;
pub(crate) mod store;

pub trait AsSlice<T> {
    fn as_slice(&self) -> &[T];
}

impl<S, T> AsSlice<T> for S
where
    S: AsRef<[T]>,
{
    fn as_slice(&self) -> &[T] {
        self.as_ref()
    }
}

pub trait AsSliceMut<T> {
    fn as_slice_mut(&mut self) -> &mut [T];
}

impl<S, T> AsSliceMut<T> for S
where
    S: AsMut<[T]>,
{
    fn as_slice_mut(&mut self) -> &mut [T] {
        self.as_mut()
    }
}

pub(crate) mod prelude {
    pub use super::eval::*;
    pub use super::gradient::*;
    pub use super::prop::*;
    pub use super::scalar::*;
    pub use super::store::*;
    pub use super::{AsSlice, AsSliceMut};
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

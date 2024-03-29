/*
    Appellation: specs <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod ndtensor;
pub mod scalar;

/// [Affine] describes a type of geometric transformation which preserves
/// lines and parallelisms.
///
/// ### General Formula
/// f(x) = A * x + b
pub trait Affine<T> {
    type Output;

    fn affine(&self, mul: T, add: T) -> Self::Output;
}

impl<A, B, C, D> Affine<B> for A
where
    A: std::ops::Mul<B, Output = C>,
    C: std::ops::Add<B, Output = D>,
    Self: Clone,
{
    type Output = D;

    fn affine(&self, mul: B, add: B) -> Self::Output {
        self.clone() * mul + add
    }
}

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
    use super::Affine;
    use num::Complex;

    #[test]
    fn test_affine() {
        let a = 3f64;
        let b = 4f64;
        let c = 5f64;

        let exp = 17f64;

        assert_eq!(a.affine(b, c), exp);

        let a = Complex::<f64>::new(3.0, 0.0);
        let b = 4f64;
        let c = 5f64;

        let exp = Complex::<f64>::new(17.0, 0.0);

        assert_eq!(a.affine(b, c), exp);
    }

    #[test]
    fn test_scalar() {
        let a = 3f64;
        let b = 4f64;

        assert_eq!(Scalar::sqr(a), 9f64);
        assert_eq!(Scalar::sqrt(b), 2f64);
    }
}

/*
    Appellation: specs <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::iter::Sum;
use core::ops;

/// [Affine] describes a type of geometric transformation which preserves
/// lines and parallelisms.
///
/// ### General Formula
/// f(x) = A * x + b
pub trait Affine<A, B> {
    type Output;

    fn affine(&self, mul: A, add: B) -> Self::Output;
}

impl<S, A, B, C> Affine<A, B> for S
where
    S: Clone + ops::Mul<A, Output = C>,
    C: ops::Add<B, Output = C>,
{
    type Output = C;

    fn affine(&self, mul: A, add: B) -> Self::Output {
        self.clone() * mul + add
    }
}

/// Inversion
///
/// The inverse of a matrix is a matrix that, when multiplied with the original matrix, gives the
/// identity matrix.
pub trait Inverse {
    type Output;

    fn inv(&self) -> Self::Output;
}

/// Matrix multiplication
pub trait Matmul<Rhs = Self> {
    type Output;

    fn matmul(&self, rhs: &Rhs) -> Self::Output;
}

impl<T> Matmul for Vec<T>
where
    T: Copy + ops::Mul<Output = T> + Sum,
{
    type Output = T;

    fn matmul(&self, rhs: &Self) -> Self::Output {
        self.iter().zip(rhs.iter()).map(|(a, b)| *a * *b).sum()
    }
}

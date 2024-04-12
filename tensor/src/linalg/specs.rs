/*
    Appellation: specs <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::iter::Sum;
use core::ops::{Add, Mul};

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
    S: Clone + Mul<A, Output = C>,
    C: Add<B, Output = C>,
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

impl<S, T> Inverse for S
where
    S: Clone + num::traits::Inv<Output = T>,
{
    type Output = T;

    fn inv(&self) -> Self::Output {
        self.clone().inv()
    }
}

/// Matrix multiplication
pub trait Matmul<Rhs = Self> {
    type Output;

    fn matmul(&self, rhs: &Rhs) -> Self::Output;
}

impl<A, B, C> Matmul<Vec<B>> for Vec<A>
where
    A: Clone + Mul<B, Output = C>,
    B: Clone,
    C: Sum,
{
    type Output = C;

    fn matmul(&self, rhs: &Vec<B>) -> Self::Output {
        self.iter()
            .cloned()
            .zip(rhs.iter().cloned())
            .map(|(a, b)| a * b)
            .sum()
    }
}

// impl_matmul!(Vec, Vec, Vec);

/*
    Appellation: linalg <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Linear Algebra
//!
//!
pub mod uplo;

use crate::shape::Axis;

pub trait Inverse {
    fn inv(&self) -> Self;
}

/// Matrix multiplication
pub trait Matmul<Rhs = Self> {
    type Output;

    fn matmul(&self, rhs: &Rhs) -> Self::Output;
}

pub trait SwapAxes {
    fn swap_axes(&self, swap: Axis, with: Axis) -> Self;
}

pub(crate) mod prelude {
    pub use super::uplo::UPLO;
    pub use super::{Inverse, Matmul, SwapAxes};
}

#[cfg(test)]
mod tests {}

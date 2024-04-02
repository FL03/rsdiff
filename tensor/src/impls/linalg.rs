/*
    Appellation: linalg <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! Implementations for linear algebra operations.
//!
//!
use crate::prelude::{Matmul, Scalar, ShapeError, TensorError, TensorExpr, TensorResult};
use crate::tensor::{self, TensorBase};
use acme::prelude::UnaryOp;
use num::traits::{Num, NumAssign, Zero};

fn inverse_impl<T>(matrix: &TensorBase<T>) -> TensorResult<TensorBase<T>>
where
    T: Copy + Num + NumAssign + PartialOrd,
{
    let op = TensorExpr::unary(matrix.clone(), UnaryOp::Inv);
    let rows = matrix.nrows();
    let cols = matrix.ncols();

    if !matrix.is_square() {
        return Err(ShapeError::IncompatibleShapes.into()); // Matrix must be square for inversion
    }

    let identity = TensorBase::eye(rows);

    // Construct an augmented matrix by concatenating the original matrix with an identity matrix
    let mut aug = TensorBase::zeros((rows, 2 * cols));
    let acols = 2 * cols;
    // aug.slice_mut(s![.., ..cols]).assign(matrix);
    for i in 0..rows {
        for j in 0..cols {
            aug[[i, j]] = matrix[[i, j]];
        }
        for j in cols..acols {
            aug[[i, j]] = identity[[i, j - cols]];
        }
    }

    // Perform Gaussian elimination to reduce the left half to the identity matrix
    for i in 0..rows {
        let pivot = aug[[i, i]];

        if pivot == T::zero() {
            return Err(TensorError::Singular); // Matrix is singular
        }

        for j in 0..(2 * cols) {
            aug[[i, j]] = aug[[i, j]] / pivot;
        }

        for j in 0..rows {
            if i != j {
                let am = aug.clone();
                let factor = aug[[j, i]];
                for k in 0..(2 * cols) {
                    aug[[j, k]] -= factor * am[[i, k]];
                }
            }
        }
    }

    // Extract the inverted matrix from the augmented matrix
    let mut inverted = matrix.zeros_like().with_op(op.into());
    for i in 0..rows {
        for j in 0..cols {
            inverted[[i, j]] = aug[[i, j + cols]];
        }
    }

    Ok(inverted.to_owned())
}
/// Returns the lower triangular portion of a matrix.
pub fn tril<T>(a: &TensorBase<T>) -> TensorBase<T>
where
    T: Clone + Zero,
{
    let mut out = a.clone();
    for i in 0..a.shape()[0] {
        for j in i + 1..a.shape()[1] {
            out[[i, j]] = T::zero();
        }
    }
    out
}
/// Returns the upper triangular portion of a matrix.
pub fn triu<T>(a: &TensorBase<T>) -> TensorBase<T>
where
    T: Clone + Zero,
{
    let mut out = a.clone();
    for i in 0..a.shape()[0] {
        for j in 0..i {
            out[[i, j]] = T::zero();
        }
    }
    out
}

impl<T> TensorBase<T>
where
    T: Copy,
{
    pub fn diag(&self) -> Self {
        let rank = *self.rank();

        let store = (0..rank).map(|i| self[vec![i; rank]]).collect::<Vec<T>>();
        tensor::from_vec_with_kind(false, self.shape().diagonalize(), store)
    }
}

impl<T> TensorBase<T>
where
    T: Copy + Num + NumAssign + PartialOrd,
{
    pub fn inv(&self) -> TensorResult<Self> {
        inverse_impl(self)
    }
}

impl<T> Matmul<TensorBase<T>> for TensorBase<T>
where
    T: Scalar,
{
    type Output = Self;

    fn matmul(&self, other: &Self) -> Self {
        let shape = self.shape().matmul_shape(&other.shape()).unwrap();
        let mut result = vec![T::zero(); shape.size()];

        for i in 0..self.shape()[0] {
            for j in 0..other.shape()[1] {
                for k in 0..self.shape()[1] {
                    result[i * other.shape()[1] + j] +=
                        self.data[i * self.shape()[1] + k] * other.data[k * other.shape()[1] + j];
                }
            }
        }
        let op = TensorExpr::matmul(self.clone(), other.clone());
        tensor::from_vec_with_op(false, op, shape, result)
    }
}

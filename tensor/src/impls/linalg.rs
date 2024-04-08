/*
    Appellation: linalg <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! Implementations for linear algebra operations.
//!
//!
use crate::linalg::{Inverse, Matmul};
use crate::prelude::{Scalar, ShapeError, TensorError, TensorExpr, TensorResult};
use crate::tensor::{self, TensorBase};
use acme::prelude::UnaryOp;
use num::traits::{Num, NumAssign};

fn inverse_impl<T>(tensor: &TensorBase<T>) -> TensorResult<TensorBase<T>>
where
    T: Copy + NumAssign + PartialOrd,
{
    let op = TensorExpr::unary(tensor.clone(), UnaryOp::Inv);
    let n = tensor.nrows();

    if !tensor.is_square() {
        return Err(ShapeError::NotSquare.into()); // Matrix must be square for inversion
    }

    let eye = TensorBase::eye(n);

    // Construct an augmented matrix by concatenating the original matrix with an identity matrix
    let mut aug = TensorBase::zeros((n, 2 * n));
    // aug.slice_mut(s![.., ..cols]).assign(matrix);
    for i in 0..n {
        for j in 0..n {
            aug[[i, j]] = tensor[[i, j]];
        }
        for j in n..(2 * n) {
            aug[[i, j]] = eye[[i, j - n]];
        }
    }

    // Perform Gaussian elimination to reduce the left half to the identity matrix
    for i in 0..n {
        let pivot = aug[[i, i]];

        if pivot == T::zero() {
            return Err(TensorError::Singular); // Matrix is singular
        }

        for j in 0..(2 * n) {
            aug[[i, j]] = aug[[i, j]] / pivot;
        }

        for j in 0..n {
            if i != j {
                let am = aug.clone();
                let factor = aug[[j, i]];
                for k in 0..(2 * n) {
                    aug[[j, k]] -= factor * am[[i, k]];
                }
            }
        }
    }

    // Extract the inverted matrix from the augmented matrix
    let mut inv = tensor.zeros_like().with_op(op.into());
    for i in 0..n {
        for j in 0..n {
            inv[[i, j]] = aug[[i, j + n]];
        }
    }

    Ok(inv.to_owned())
}

impl<T> TensorBase<T>
where
    T: Copy,
{
    /// Creates a new tensor containing only the diagonal elements of the original tensor.
    pub fn diag(&self) -> Self {
        let n = self.nrows();
        Self::from_shape_iter(self.shape().diag(), (0..n).map(|i| self[vec![i; n]]))
    }
    /// Find the inverse of the tensor
    ///
    /// # Errors
    ///
    /// Returns an error if the matrix is not square or if the matrix is singular.
    ///
    pub fn inv(&self) -> TensorResult<Self>
    where
        T: NumAssign + PartialOrd,
    {
        inverse_impl(self)
    }
    /// Compute the trace of the matrix.
    /// The trace of a matrix is the sum of the diagonal elements.
    pub fn trace(&self) -> TensorResult<T>
    where
        T: Num,
    {
        if !self.is_square() {
            return Err(ShapeError::NotSquare.into());
        }
        let n = self.nrows();
        let trace = (0..n).fold(T::zero(), |acc, i| acc + self[[i, i]]);
        Ok(trace)
    }
}

impl<T> Inverse for TensorBase<T>
where
    T: Copy + Num + NumAssign + PartialOrd,
{
    type Output = TensorResult<Self>;

    fn inv(&self) -> Self::Output {
        inverse_impl(self)
    }
}

impl<T> Matmul<TensorBase<T>> for TensorBase<T>
where
    T: Scalar,
{
    type Output = Self;

    fn matmul(&self, other: &Self) -> Self {
        let sc = |m: usize, n: usize| m * self.ncols() + n;
        let oc = |m: usize, n: usize| m * other.ncols() + n;

        let shape = self.shape().matmul_shape(&other.shape()).unwrap();
        let mut result = vec![T::zero(); shape.size()];

        for i in 0..self.nrows() {
            for j in 0..other.ncols() {
                for k in 0..self.ncols() {
                    result[oc(i, j)] += self.data[sc(i, k)] * other.data[oc(k, j)];
                }
            }
        }
        let op = TensorExpr::matmul(self.clone(), other.clone());
        tensor::from_vec_with_op(false, op, shape, result)
    }
}

#[allow(dead_code)]
macro_rules! multi_for {
    ($($key:ident in $iter:expr =>)* $expr:expr) => {
        for $key in $iter {
            multi_for!($($key in $iter =>)* $expr);
        }
    };
    ($key:ident in $iter:expr => $expr:expr) => {
        for $key in $iter {
            $expr
        }
    };
}

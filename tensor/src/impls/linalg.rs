/*
    Appellation: linalg <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! Implementations for linear algebra operations.
//!
//!
use crate::prelude::{Matmul, Scalar, ShapeError, TensorError, TensorExpr, TensorResult};
use crate::tensor::*;
use acme::prelude::UnaryOp;
use num::traits::{Num, Signed};

pub fn inverse<T>(tensor: &TensorBase<T>) -> TensorResult<TensorBase<T>>
where
    T: Copy + Num + PartialOrd + Signed,
{
    if !tensor.shape().is_square() {
        return Err(ShapeError::InvalidShape.into());
    }
    let shape = tensor.shape();
    let n = *shape.first().unwrap();

    let mut data = tensor.data().to_vec();
    let mut inverse = vec![T::zero(); n * n];

    for i in 0..n {
        inverse[(i * n) + i] = T::one();
    }

    let mut permutation = vec![0; n];
    for i in 0..n {
        permutation[i] = i;
    }

    for i in 0..n {
        let mut max_row = i;
        for j in i + 1..n {
            if data[(j * n) + i].abs() > data[(max_row * n) + i].abs() {
                max_row = j;
            }
        }

        if data[(max_row * n) + i].is_zero() {
            return Err(TensorError::Singular); // Matrix is singular
        }

        if max_row != i {
            for j in 0..n {
                data.swap((max_row * n) + j, (i * n) + j);
                inverse.swap((max_row * n) + j, (i * n) + j);
            }
            permutation.swap(max_row, i);
        }

        let pivot = data[(i * n) + i];
        for j in 0..n {
            data[(i * n) + j] = data[(i * n) + j] / pivot;
            inverse[(i * n) + j] = inverse[(i * n) + j] / pivot;
        }

        for j in 0..n {
            if j != i {
                let factor = data[(j * n) + i];
                for k in 0..n {
                    data[(j * n) + k] = data[(j * n) + k] - data[(i * n) + k] * factor;
                    inverse[(j * n) + k] = inverse[(j * n) + k] - inverse[(i * n) + k] * factor;
                }
            }
        }
    }

    let mut res = vec![T::zero(); n * n];
    for i in 0..n {
        for j in 0..n {
            res[(i * n) + permutation[j]] = inverse[(i * n) + j];
        }
    }
    let op = TensorExpr::unary(tensor.clone(), UnaryOp::Inv);
    let tensor = from_vec_with_op(false, op, shape, res);
    Ok(tensor)
}

impl<T> TensorBase<T>
where
    T: Copy + Num + PartialOrd + Signed,
{
    pub fn diag(&self) -> Self
    where
        T: Clone,
    {
        let rank = *self.rank();

        let store = (0..rank).map(|i| self[vec![i; rank]]).collect::<Vec<T>>();
        from_vec(false, self.shape().diagonalize(), store)
    }
    pub fn inv(&self) -> TensorResult<Self> {
        inverse(self)
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
                        self.store[i * self.shape()[1] + k] * other.store[k * other.shape()[1] + j];
                }
            }
        }
        let op = TensorExpr::matmul(self.clone(), other.clone());
        from_vec_with_op(false, op, shape, result)
    }
}

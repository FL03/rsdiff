/*
    Appellation: linalg <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! Implementations for linear algebra operations.
//!
//!
use crate::prelude::{Matmul, Scalar, TensorOp, TensorResult};
use crate::shape::ShapeError;
use crate::tensor::*;

pub(crate) fn matmul<T>(lhs: &TensorBase<T>, rhs: &TensorBase<T>) -> TensorResult<TensorBase<T>>
where
    T: Scalar,
{
    if lhs.shape().rank() != rhs.shape().rank() {
        return Err(ShapeError::IncompatibleShapes.into());
    }

    let lhs_shape = lhs.shape().clone();
    let lhs_n = *lhs_shape.last().unwrap();
    // let lhs_m = lhs_shape.
    let rhs_shape = rhs.shape().clone();

    let shape = lhs_shape.matmul_shape(rhs.shape()).unwrap();
    let mut result = vec![T::zero(); shape.elements()];

    for i in 0..lhs_shape[0] {
        for j in 0..rhs_shape[1] {
            for k in 0..lhs_n {
                let pos = i * rhs_shape[1] + j;
                let left = i * lhs_n + k;
                let right = k * rhs_shape[1] + j;
                result[pos] += lhs.store[left] * rhs.store[right];
            }
        }
    }
    let op = TensorOp::Matmul(Box::new(lhs.clone()), Box::new(rhs.clone()));
    Ok(from_vec_with_op(op, shape, result))
}

impl<T> Matmul<TensorBase<T>> for TensorBase<T>
where
    T: Scalar,
{
    type Output = Self;

    fn matmul(&self, other: &Self) -> Self {
        let shape = self.shape().matmul_shape(other.shape()).unwrap();
        let mut result = vec![T::zero(); shape.elements()];

        for i in 0..self.shape()[0] {
            for j in 0..other.shape()[1] {
                for k in 0..self.shape()[1] {
                    result[i * other.shape()[1] + j] +=
                        self.store[i * self.shape()[1] + k] * other.store[k * other.shape()[1] + j];
                }
            }
        }
        let op = TensorOp::Matmul(Box::new(self.clone()), Box::new(other.clone()));
        from_vec_with_op(op, shape, result)
    }
}

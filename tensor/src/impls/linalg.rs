/*
    Appellation: linalg <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! Implementations for linear algebra operations.
//!
//!
use crate::prelude::{Matmul, Scalar, TensorOp};
use crate::tensor::*;

impl<T> Matmul<TensorBase<T>> for TensorBase<T>
where
    T: Scalar,
{
    type Output = Self;

    fn matmul(&self, other: &Self) -> Self {
        let shape = self.shape().matmul_shape(other.shape()).unwrap();
        let mut result = vec![T::zero(); shape.size()];

        for i in 0..self.shape()[0] {
            for j in 0..other.shape()[1] {
                for k in 0..self.shape()[1] {
                    result[i * other.shape()[1] + j] +=
                        self.store[i * self.shape()[1] + k] * other.store[k * other.shape()[1] + j];
                }
            }
        }
        let op = TensorOp::matmul(self.clone(), other.clone());
        from_vec_with_op(false, op, shape, result)
    }
}

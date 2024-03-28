/*
    Appellation: utils <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Utilities
//!
//!
use crate::prelude::{Scalar, TensorOp, TensorResult};
use crate::shape::ShapeError;
use crate::tensor::{from_vec_with_op, TensorBase};

pub fn matmul<T>(lhs: &TensorBase<T>, rhs: &TensorBase<T>) -> TensorResult<TensorBase<T>>
where
    T: Scalar,
{
    if lhs.shape().rank() != rhs.shape().rank() {
        return Err(ShapeError::IncompatibleShapes.into());
    }

    let shape = lhs.shape().matmul_shape(rhs.shape()).unwrap();
    let mut result = vec![T::zero(); shape.size()];

    for i in 0..lhs.shape().nrows() {
        for j in 0..rhs.shape().ncols() {
            for k in 0..lhs.shape().ncols() {
                let pos = i * rhs.shape().ncols() + j;
                let left = i * lhs.shape().ncols() + k;
                let right = k * rhs.shape().ncols() + j;
                result[pos] += lhs.store[left] * rhs.store[right];
            }
        }
    }
    let op = TensorOp::matmul(lhs.clone(), rhs.clone());
    let tensor = from_vec_with_op(false, op, shape, result);
    Ok(tensor)
}

pub fn dot_product<T>(lhs: &TensorBase<T>, rhs: &TensorBase<T>) -> TensorResult<TensorBase<T>>
where
    T: Scalar,
{
    if lhs.shape().rank() != rhs.shape().rank() {
        return Err(ShapeError::IncompatibleShapes.into());
    }

    let shape = lhs.shape().matmul_shape(rhs.shape()).unwrap();
    let mut result = vec![T::zero(); shape.size()];

    for i in 0..lhs.shape().nrows() {
        for j in 0..rhs.shape().ncols() {
            for k in 0..lhs.shape().ncols() {
                let pos = i * rhs.shape().ncols() + j;
                let left = i * lhs.shape().ncols() + k;
                let right = k * rhs.shape().ncols() + j;
                result[pos] += lhs.store[left] * rhs.store[right];
            }
        }
    }
    let op = TensorOp::matmul(lhs.clone(), rhs.clone());
    let tensor = from_vec_with_op(false, op, shape, result);
    Ok(tensor)
}

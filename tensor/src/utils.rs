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
    let mut result = vec![T::zero(); shape.elements()];

    for i in 0..lhs.shape().rows() {
        for j in 0..rhs.shape().columns() {
            for k in 0..lhs.shape().columns() {
                let pos = i * rhs.shape().columns() + j;
                let left = i * lhs.shape().columns() + k;
                let right = k * rhs.shape().columns() + j;
                result[pos] += lhs.store[left] * rhs.store[right];
            }
        }
    }
    let op = TensorOp::Matmul(Box::new(lhs.clone()), Box::new(rhs.clone()));
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
    let mut result = vec![T::zero(); shape.elements()];

    for i in 0..lhs.shape().rows() {
        for j in 0..rhs.shape().columns() {
            for k in 0..lhs.shape().columns() {
                let pos = i * rhs.shape().columns() + j;
                let left = i * lhs.shape().columns() + k;
                let right = k * rhs.shape().columns() + j;
                result[pos] += lhs.store[left] * rhs.store[right];
            }
        }
    }
    let op = TensorOp::Matmul(Box::new(lhs.clone()), Box::new(rhs.clone()));
    let tensor = from_vec_with_op(false, op, shape, result);
    Ok(tensor)
}

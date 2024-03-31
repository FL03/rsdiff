/*
    Appellation: utils <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Utilities
//!
//!
use crate::prelude::{Scalar, TensorExpr, TensorResult};
use crate::shape::ShapeError;
use crate::tensor::{from_vec_with_op, TensorBase};

pub fn matmul<T>(lhs: &TensorBase<T>, rhs: &TensorBase<T>) -> TensorResult<TensorBase<T>>
where
    T: Scalar,
{
    if lhs.shape().rank() != rhs.shape().rank() {
        return Err(ShapeError::DimensionMismatch.into());
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
    let op = TensorExpr::matmul(lhs.clone(), rhs.clone());
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

    let shape = lhs.shape().matmul_shape(&rhs.shape()).unwrap();
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
    let op = TensorExpr::matmul(lhs.clone(), rhs.clone());
    let tensor = from_vec_with_op(false, op, shape, result);
    Ok(tensor)
}

macro_rules! izip {
    // @closure creates a tuple-flattening closure for .map() call. usage:
    // @closure partial_pattern => partial_tuple , rest , of , iterators
    // eg. izip!( @closure ((a, b), c) => (a, b, c) , dd , ee )
    ( @closure $p:pat => $tup:expr ) => {
        |$p| $tup
    };

    // The "b" identifier is a different identifier on each recursion level thanks to hygiene.
    ( @closure $p:pat => ( $($tup:tt)* ) , $_iter:expr $( , $tail:expr )* ) => {
        izip!(@closure ($p, b) => ( $($tup)*, b ) $( , $tail )*)
    };

    // unary
    ($first:expr $(,)*) => {
        IntoIterator::into_iter($first)
    };

    // binary
    ($first:expr, $second:expr $(,)*) => {
        izip!($first)
            .zip($second)
    };

    // n-ary where n > 2
    ( $first:expr $( , $rest:expr )* $(,)* ) => {
        izip!($first)
            $(
                .zip($rest)
            )*
            .map(
                izip!(@closure a => (a) $( , $rest )*)
            )
    };
}

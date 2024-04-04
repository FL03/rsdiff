/*
    Appellation: utils <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Scalar, TensorExpr, TensorResult};
use crate::shape::ShapeError;
use crate::tensor::{from_vec_with_op, TensorBase};
use num::Zero;

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
                result[pos] += lhs.data[left] * rhs.data[right];
            }
        }
    }
    let op = TensorExpr::matmul(lhs.clone(), rhs.clone());
    let tensor = from_vec_with_op(false, op, shape, result);
    Ok(tensor)
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

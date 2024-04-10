/*
    Appellation: utils <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Scalar, TensorExpr, TensorResult};
use crate::shape::{ShapeError, Stride};
use crate::tensor::{from_vec_with_op, TensorBase};

pub(crate) fn coordinates_to_index(coords: impl AsRef<[usize]>, strides: &Stride) -> usize {
    coords
        .as_ref()
        .iter()
        .zip(strides.iter())
        .fold(0, |acc, (&i, &s)| acc + i * s)
}

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

macro_rules! i {
    ($($x:expr),*) => {
        vec![$($x),*]
    };

}

macro_rules! impl_partial_eq {
    ($s:ident -> $cmp:tt: [$($t:ty),*]) => {
        $(
            impl_partial_eq!($s -> $cmp, $t);
        )*
    };
    ($s:ident -> $cmp:tt, $t:ty) => {
        impl PartialEq<$t> for $s {
            fn eq(&self, other: &$t) -> bool {
                self.$cmp == *other
            }
        }

        impl PartialEq<$s> for $t {
            fn eq(&self, other: &$s) -> bool {
                *self == other.$cmp
            }
        }
    };
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

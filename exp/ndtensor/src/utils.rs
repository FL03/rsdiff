/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use ndarray::{Array, Dimension, IntoDimension};
use num::Float;
use std::hash::{DefaultHasher, Hash, Hasher};

/// Hashes a dimension using the [DefaultHasher].
pub fn hash_dim<D>(dim: impl IntoDimension<Dim = D>) -> u64
where
    D: Dimension,
{
    let dim = dim.into_dimension();
    let mut s = DefaultHasher::new();
    for i in dim.slice() {
        i.hash(&mut s);
    }
    s.finish()
}

#[allow(dead_code)]
pub(crate) fn linspace<A, D>(dim: impl IntoDimension<Dim = D>) -> Array<A, D>
where
    A: Float,
    D: Dimension,
{
    let dim = dim.into_dimension();
    let dview = dim.as_array_view();
    let n = dview.product();
    Array::linspace(A::zero(), A::from(n).unwrap() - A::one(), n)
        .into_shape(dim)
        .expect("linspace err")
}

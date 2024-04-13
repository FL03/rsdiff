/*
    Appellation: tensor <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::TensorExpr;
use acme::prelude::AtomicId;
use core::fmt;
use ndarray::{ArrayBase, Data, Dimension, IxDyn, RawData};

pub struct Tensor<S>
where
    S: RawData,
{
    id: AtomicId,
    data: ArrayBase<S, IxDyn>,
    op: Option<TensorExpr<S>>,
}

impl<S> Tensor<S>
where
    S: RawData,
{
    pub fn new<D>(data: ArrayBase<S, D>) -> Self
    where
        D: Dimension,
    {
        Tensor {
            id: AtomicId::new(),
            data: data.into_dyn(),
            op: None,
        }
    }

    /// Returns the unique identifier of the tensor.
    pub const fn id(&self) -> AtomicId {
        self.id
    }
    /// Gets an immutable reference to the operations of the tensor.
    pub fn op(&self) -> Option<&TensorExpr<S>> {
        self.op.as_ref()
    }
}

impl<S> fmt::Debug for Tensor<S>
where
    S: Data,
    S::Elem: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

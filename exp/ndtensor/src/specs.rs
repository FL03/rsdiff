/*
    Appellation: specs <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{ArrayBase, Dimension, IxDyn};
use crate::Tensor;
use acme::prelude::Scalar;
use ndarray::{Ix0, RawData};

pub trait NdTensor<S, D = IxDyn>
where
    D: Dimension,
    S: RawData,
{
    fn data(&self) -> ArrayBase<S, D>;

    fn dim(&self) -> D;

    fn rank(&self) -> usize {
        D::NDIM.unwrap_or(self.dim().slice().len())
    }
}

pub trait ScalarExt: Scalar {
    fn into_tensor(self) -> Tensor<Self, Ix0> {
        Tensor::from_scalar(self)
    }
    fn sigmoid(self) -> Self {
        (Self::one() + self.neg().exp()).recip()
    }
}

impl<S> ScalarExt for S where S: Scalar {}

/*
    Appellation: scalar <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::tensor::TensorBase;
use acme::prelude::Scalar;

pub trait ScalarExt: Scalar {
    fn into_tensor(self) -> TensorBase<Self> {
        TensorBase::from_scalar(self)
    }
    fn sigmoid(self) -> Self {
        (Self::one() + self.neg().exp()).recip()
    }
}

impl<S> ScalarExt for S where S: Scalar {}

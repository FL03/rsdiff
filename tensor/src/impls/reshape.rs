/*
    Appellation: reshape <impls>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::IntoShape;
use crate::tensor::TensorBase;

impl<T> TensorBase<T>
where
    T: Clone + Default,
{
    pub fn broadcast(&self, shape: impl IntoShape) -> Self {
        let shape = shape.into_shape();

        let _diff = *self.shape().rank() - *shape.rank();

        unimplemented!()
    }

    pub fn reshape(&self, shape: impl IntoShape) -> Self {
        let _shape = shape.into_shape();

        unimplemented!()
    }
}

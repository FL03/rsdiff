/*
    Appellation: reshape <impls>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{IntoShape, ShapeError, TensorResult};
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

    pub fn reshape(self, shape: impl IntoShape) -> TensorResult<Self> {
        let mut tensor = self;
        let shape = shape.into_shape();
        if tensor.elements() != shape.elements() {
            return Err(ShapeError::MismatchedElements.into());
        }

        tensor.layout.shape = shape;
        Ok(tensor)
    }
}

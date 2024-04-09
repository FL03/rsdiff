/*
    Appellation: reshape <impls>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{TensorExpr, TensorId, TensorResult};
use crate::shape::{Axis, IntoShape, ShapeError};
use crate::tensor::TensorBase;

impl<T> TensorBase<T>
where
    T: Clone,
{
    /// coerce the tensor to act like a larger shape.
    /// This method doesn't change the underlying data, but it does change the layout.
    pub fn broadcast(&self, shape: impl IntoShape) -> Self {
        let layout = self.layout().broadcast_as(shape).unwrap();
        let op = TensorExpr::broadcast(self.clone(), layout.shape().clone());
        Self {
            id: TensorId::new(),
            kind: self.kind(),
            layout,
            op: op.into(),
            data: self.data().clone(),
        }
    }
    #[doc(hidden)]
    pub fn pad(&self, shape: impl IntoShape, _with: T) -> Self {
        let shape = shape.into_shape();

        let _diff = *self.shape().rank() - *shape.rank();

        todo!()
    }
    /// Swap two axes in the tensor.
    pub fn swap_axes(&self, swap: Axis, with: Axis) -> Self {
        let op = TensorExpr::swap_axes(self.clone(), swap, with);

        let layout = self.layout().clone().swap_axes(swap, with);
        let shape = self.layout.shape();
        let mut data = self.data.to_vec();

        for i in 0..shape[swap] {
            for j in 0..shape[with] {
                let scope = self.layout.index([i, j]);
                let target = layout.index([j, i]);
                data[target] = self.data()[scope].clone();
            }
        }

        TensorBase {
            id: TensorId::new(),
            kind: self.kind.clone(),
            layout,
            op: op.into(),
            data: data.clone(),
        }
    }
    /// Transpose the tensor.
    pub fn t(&self) -> Self {
        let op = TensorExpr::transpose(self.clone());

        let layout = self.layout().clone().reverse_axes();
        TensorBase {
            id: TensorId::new(),
            kind: self.kind(),
            layout,
            op: op.into(),
            data: self.data().clone(),
        }
    }
    /// Reshape the tensor
    /// returns an error if the new shape specifies a different number of elements.
    pub fn reshape(self, shape: impl IntoShape) -> TensorResult<Self> {
        let shape = shape.into_shape();
        if self.size() != shape.size() {
            return Err(ShapeError::MismatchedElements.into());
        }

        let mut tensor = self;

        tensor.layout.reshape(shape);

        Ok(tensor)
    }
}

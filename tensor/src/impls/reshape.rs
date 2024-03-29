/*
    Appellation: reshape <impls>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{TensorExpr, TensorId, TensorResult};
use crate::shape::{Axis, IntoShape, ShapeError};
use crate::tensor::TensorBase;

impl<T> TensorBase<T>
where
    T: Clone + Default,
{
    pub fn broadcast(&self, shape: impl IntoShape) -> Self {
        let layout = self.layout.broadcast_as(shape).unwrap();

        Self {
            id: TensorId::new(),
            kind: self.kind.clone(),
            layout,
            op: self.op.clone(),
            store: self.store.clone(),
        }
    }

    pub fn pad(&self, shape: impl IntoShape, _with: T) -> Self {
        let shape = shape.into_shape();

        let _diff = *self.shape().rank() - *shape.rank();

        unimplemented!()
    }

    ///
    pub fn swap_axes(&self, swap: Axis, with: Axis) -> Self {
        let op = TensorExpr::transpose(self.clone(), swap, with);

        let layout = self.layout().clone().transpose(swap, with);
        let shape = self.layout.shape();
        let mut data = self.store.to_vec();

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
            store: data.clone(),
        }
    }
    /// Transpose the tensor.
    pub fn t(&self) -> Self {
        let (a, b) = (Axis(0), Axis(1));
        let op = TensorExpr::transpose(self.clone(), a, b);

        let layout = self.layout().clone().transpose(a, b);
        let shape = self.layout.shape();
        let mut data = self.store.to_vec();

        for i in 0..shape[a] {
            for j in 0..shape[b] {
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
            store: data.clone(),
        }
    }

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

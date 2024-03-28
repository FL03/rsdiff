/*
    Appellation: reshape <impls>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{BackpropOp, TensorId, TensorOp, TensorResult};
use crate::shape::{Axis, IntoShape, ShapeError};
use crate::tensor::{from_vec, TensorBase};

impl<T> TensorBase<T>
where
    T: Clone + Default,
{
    pub fn broadcast(&self, shape: impl IntoShape) -> Self {
        let shape = shape.into_shape();

        let _diff = *self.shape().rank() - *shape.rank();

        unimplemented!()
    }

    pub fn pad(&self, shape: impl IntoShape, _with: T) -> Self {
        let shape = shape.into_shape();

        let _diff = *self.shape().rank() - *shape.rank();

        unimplemented!()
    }

    ///
    pub fn swap_axes(&self, swap: Axis, with: Axis) -> TensorResult<Self> {
        let layout = self.layout().swap_axes(swap, with);

        let shape = self.shape();
        let mut res = self.data().clone();

        for i in 0..shape[swap] {
            for j in 0..shape[with] {
                let target = self.layout.position(&[i, j])?;
                let dest = layout.position(&[j, i])?;
                res[dest] = self.data()[target].clone();
            }
        }

        let tensor = crate::new(false, None, layout.shape(), res);
        Ok(tensor)
    }
    /// Transpose the tensor.
    pub fn t(&self) -> TensorBase<T> {
        let (a, b) = (Axis(0), Axis(1));
        let op = TensorOp::transpose(self.clone(), a, b);

        let layout = self.layout().clone().transpose(a, b);
        let shape = self.layout.shape();
        let mut data = self.store.to_vec();

        for i in 0..shape[a] {
            for j in 0..shape[b] {
                let scope = self.layout.select([i, j]);
                let target = layout.select([j, i]);
                println!("Swapping {:?} with {:?}", scope, target);
                data[target] = self[&[i, j]].clone();
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

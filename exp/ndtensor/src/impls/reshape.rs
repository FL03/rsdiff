/*
    Appellation: reshape <impls>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::TensorExpr;
use crate::TensorBase;
use ndarray::{DataOwned, Dimension, IntoDimension, RawData, ShapeError};

impl<A, S, D> TensorBase<S, D>
where
    D: Dimension,
    S: RawData<Elem = A>,
{
    pub fn into_shape<D2>(self, shape: D2) -> Result<TensorBase<S, D2::Dim>, ShapeError>
    where
        D2: IntoDimension,
    {
        let data = self.data.into_shape(shape)?;
        Ok(TensorBase {
            id: self.id,
            ctx: self.ctx,
            data,
            op: self.op,
        })
    }

    pub fn reshape<D2>(&self, _shape: D2) -> Result<TensorBase<S, D2::Dim>, ShapeError>
    where
        D2: IntoDimension,
    {
        unimplemented!("reshape")
    }
    /// Transpose the tensor.
    pub fn t(&self) -> crate::Tensor<A, D>
    where
        A: Clone,
        S: DataOwned,
    {
        TensorBase {
            id: self.id,
            ctx: self.ctx,
            data: self.data().t().to_owned(),
            op: TensorExpr::transpose(Box::new(self.to_owned().into_dyn())).into(),
        }
    }
}

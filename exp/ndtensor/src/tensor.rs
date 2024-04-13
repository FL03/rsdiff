/*
    Appellation: tensor <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use crate::ops::TensorExpr;
use crate::prelude::{TensorId, TensorResult};
use core::fmt;
use ndarray::{ArrayBase, Data, DataOwned, Dimension, IxDyn, RawData};

pub(crate) fn new<S, D>(data: ArrayBase<S, D>, op: Option<TensorExpr<S>>) -> Tensor<S, D>
where
    D: Dimension,
    S: RawData,
{
    Tensor {
        id: TensorId::new(),
        data,
        op,
    }
}

#[allow(dead_code)]
pub(crate) fn from_arr<S, D>(data: ArrayBase<S, D>) -> Tensor<S, D>
where
    D: Dimension,
    S: RawData,
{
    new(data, None)
}

pub struct Tensor<S, D = IxDyn>
where
    D: Dimension,
    S: RawData,
{
    id: TensorId,
    data: ArrayBase<S, D>,
    op: Option<TensorExpr<S>>,
}

impl<S, D> Tensor<S, D>
where
    D: Dimension,
    S: RawData,
{
    pub fn from_arr(data: ArrayBase<S, D>) -> Self {
        new(data, None)
    }

    pub fn from_shape_vec(shape: D, data: Vec<S::Elem>) -> TensorResult<Self>
    where
        S: DataOwned,
    {
        let data = ArrayBase::from_shape_vec(shape, data)?;
        Ok(new(data, None))
    }

    pub fn try_from_arr<D2>(data: ArrayBase<S, D2>) -> TensorResult<Self>
    where
        D2: Dimension,
    {
        let tensor = Self::from_arr(data.into_dimensionality::<D>()?);
        Ok(tensor)
    }

    /// Returns the unique identifier of the tensor.
    pub const fn id(&self) -> TensorId {
        self.id
    }
    /// Gets an immutable reference to the operations of the tensor.
    pub fn op(&self) -> Option<&TensorExpr<S>> {
        self.op.as_ref()
    }
}

impl<S> Tensor<S>
where
    S: RawData,
{
    pub fn ndtensor<D>(data: ArrayBase<S, D>) -> Self
    where
        D: Dimension,
    {
        new(data.into_dyn(), None)
    }

    pub fn to_dimensionality<D2>(self) -> TensorResult<Tensor<S, D2>>
    where
        D2: Dimension,
    {
        let data = self.data.into_dimensionality::<D2>()?;
        Ok(Tensor {
            id: self.id,
            data,
            op: self.op,
        })
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

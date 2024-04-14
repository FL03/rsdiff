/*
    Appellation: tensor <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{TensorExpr, TensorId, TensorOp, TensorResult};
use core::fmt;
use ndarray::iter::{Iter, IterMut};
use ndarray::{ArrayBase, Dimension, IxDyn};
use ndarray::{Data, DataOwned, DataShared, OwnedArcRepr, OwnedRepr, RawData, RawDataClone};

pub(crate) fn new<S1, D>(data: ArrayBase<S1, D>, op: Option<TensorExpr<S1>>) -> TensorBase<S1, D>
where
    D: Dimension,
    S1: RawData,
{
    TensorBase {
        id: TensorId::new(),
        data,
        op: TensorOp::new(op),
    }
}

#[allow(dead_code)]
pub(crate) fn from_arr<S, D>(data: ArrayBase<S, D>) -> TensorBase<S, D>
where
    D: Dimension,
    S: RawData,
{
    new(data, None)
}

pub struct TensorBase<S, D = IxDyn>
where
    D: Dimension,
    S: RawData,
{
    pub(crate) id: TensorId,
    pub(crate) data: ArrayBase<S, D>,
    pub(crate) op: TensorOp<S>,
}

impl<A, S, D> TensorBase<S, D>
where
    D: Dimension,
    S: RawData<Elem = A>,
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

    pub fn boxed(self) -> Box<TensorBase<S, D>> {
        Box::new(self)
    }

    pub const fn data(&self) -> &ArrayBase<S, D> {
        &self.data
    }

    /// Returns the unique identifier of the tensor.
    pub const fn id(&self) -> TensorId {
        self.id
    }

    pub fn into_dimensionality<D2>(self) -> TensorResult<TensorBase<S, D2>>
    where
        D2: Dimension,
    {
        let data = self.data.into_dimensionality::<D2>()?;
        Ok(TensorBase {
            id: self.id,
            data,
            op: self.op,
        })
    }

    pub fn into_dyn(self) -> TensorBase<S, IxDyn> {
        new(self.data.into_dyn(), self.op.0)
    }

    pub fn into_owned(self) -> TensorBase<OwnedRepr<A>, D>
    where
        A: Clone,
        S: DataOwned,
    {
        new(self.data.into_owned(), self.op.into_owned().0)
    }

    pub fn into_shared(self) -> TensorBase<OwnedArcRepr<A>, D>
    where
        S: DataOwned,
    {
        new(self.data.into_shared(), self.op.into_shared().0)
    }

    pub fn iter(&self) -> Iter<'_, A, D>
    where
        S: Data,
    {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, A, D>
    where
        S: ndarray::DataMut,
    {
        self.data.iter_mut()
    }
    /// Gets an immutable reference to the operations of the tensor.
    pub fn op(&self) -> Option<&TensorExpr<S>> {
        self.op.as_ref()
    }

    pub fn with_op(mut self, op: impl Into<TensorOp<S>>) -> Self {
        self.op = op.into();
        self
    }
}

impl<A, S> TensorBase<S>
where
    S: RawData<Elem = A>,
{
    pub fn ndtensor<D>(data: ArrayBase<S, D>) -> Self
    where
        D: Dimension,
    {
        new(data.into_dyn(), None)
    }
}

impl<S, D> Clone for TensorBase<S, D>
where
    D: Dimension,
    S: RawDataClone,
{
    fn clone(&self) -> Self {
        let data = self.data.clone();
        let op = self.op.clone();
        TensorBase {
            id: self.id,
            data,
            op,
        }
    }
}

impl<S, D> fmt::Debug for TensorBase<S, D>
where
    D: Dimension,
    S: Data,
    S::Elem: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl<S, D> fmt::Display for TensorBase<S, D>
where
    D: Dimension,
    S: Data,
    S::Elem: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

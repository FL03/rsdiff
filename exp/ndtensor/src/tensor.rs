/*
    Appellation: tensor <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use crate::ops::TensorExpr;
use crate::prelude::{Scalar, TensorId, TensorResult};
use acme::prelude::UnaryOp;
use core::fmt;
use ndarray::{ArrayBase, Data, DataOwned, Dimension, IxDyn, RawData};

pub(crate) fn new<S, D>(data: ArrayBase<S, D>, op: Option<TensorExpr<S>>) -> TensorBase<S, D>
where
    D: Dimension,
    S: RawData,
{
    TensorBase {
        id: TensorId::new(),
        data,
        op,
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
    id: TensorId,
    data: ArrayBase<S, D>,
    op: Option<TensorExpr<S>>,
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

    pub fn data(&self) -> &ArrayBase<S, D> {
        &self.data
    }

    /// Returns the unique identifier of the tensor.
    pub const fn id(&self) -> TensorId {
        self.id
    }

    pub fn into_dyn(self) -> TensorBase<S, IxDyn> {
        new(self.data.into_dyn(), self.op)
    }

    pub fn iter(&self) -> ndarray::iter::Iter<'_, A, D>
    where
        S: Data,
    {
        self.data.iter()
    }

    /// Gets an immutable reference to the operations of the tensor.
    pub fn op(&self) -> Option<&TensorExpr<S>> {
        self.op.as_ref()
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

    pub fn to_dimensionality<D2>(self) -> TensorResult<TensorBase<S, D2>>
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
}

impl<A, D> TensorBase<ndarray::OwnedRepr<A>, D>
where
    D: Dimension,
    A: Scalar,
{
    pub fn cos(&self) -> Self {
        let data = self.data.mapv(|x| x.cos());
        let op = TensorExpr::unary(Box::new(self.clone().into_dyn()), UnaryOp::Cos);
        new(data, Some(op))
    }

    pub fn cosh(&self) -> Self {
        let data = self.data.mapv(|x| x.cosh());
        let op = TensorExpr::unary(Box::new(self.clone().into_dyn()), UnaryOp::Cosh);
        new(data, Some(op))
    }

    pub fn exp(&self) -> Self {
        let data = self.data.mapv(|x| x.exp());
        let op = TensorExpr::unary(Box::new(self.clone().into_dyn()), UnaryOp::Exp);
        new(data, Some(op))
    }

    pub fn ln(&self) -> Self {
        let data = self.data.mapv(|x| x.ln());
        let op = TensorExpr::unary(Box::new(self.clone().into_dyn()), UnaryOp::Ln);
        new(data, Some(op))
    }

    pub fn sin(&self) -> Self {
        let data = self.data.mapv(|x| x.sin());
        let op = TensorExpr::unary(Box::new(self.clone().into_dyn()), UnaryOp::Sin);
        new(data, Some(op))
    }

    pub fn sqr(&self) -> Self {
        let data = self.data.mapv(|x| x.sqr());
        let op = TensorExpr::unary(Box::new(self.clone().into_dyn()), UnaryOp::Square);
        new(data, Some(op))
    }

    pub fn sqrt(&self) -> Self {
        let data = self.data.mapv(|x| x.sqrt());
        let op = TensorExpr::unary(Box::new(self.clone().into_dyn()), UnaryOp::Sqrt);
        new(data, Some(op))
    }

    pub fn tan(&self) -> Self {
        let data = self.data.mapv(|x| x.tan());
        let op = TensorExpr::unary(Box::new(self.clone().into_dyn()), UnaryOp::Tan);
        new(data, Some(op))
    }

    pub fn tanh(&self) -> Self {
        let data = self.data.mapv(|x| x.tanh());
        let op = TensorExpr::unary(Box::new(self.clone().into_dyn()), UnaryOp::Tanh);
        new(data, Some(op))
    }
}

impl<S, D> Clone for TensorBase<S, D>
where
    D: Dimension,
    S: ndarray::RawDataClone,
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

/*
    Appellation: tensor <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{TensorExpr, TensorId, TensorOp, TensorResult};
use crate::Context;
#[cfg(not(feature = "std"))]
use alloc::vec;
use core::borrow::{Borrow, BorrowMut};
use core::fmt;
use ndarray::iter::{Iter, IterMut};
use ndarray::*;
#[cfg(feature = "std")]
use std::vec;


pub(crate) fn new<S, D>(
    data: ArrayBase<S, D>,
    op: Option<TensorExpr<S>>,
    kind: bool,
) -> TensorBase<S, D>
where
    D: Dimension,
    S: RawData,
{
    TensorBase::new(data, op, kind)
}

pub struct TensorBase<S, D = IxDyn>
where
    D: Dimension,
    S: RawData,
{
    pub(crate) id: TensorId,
    pub(crate) ctx: Context,
    pub(crate) data: ArrayBase<S, D>,
    pub(crate) op: TensorOp<S>,
}

impl<A, S, D> TensorBase<S, D>
where
    D: Dimension,
    S: RawData<Elem = A>,
{
    pub(crate) fn new(data: ArrayBase<S, D>, op: Option<TensorExpr<S>>, kind: bool) -> Self {
        let ctx = Context::new(kind, data.ndim());
        TensorBase {
            id: TensorId::new(),
            ctx,
            data,
            op: TensorOp::new(op),
        }
    }

    pub fn boxed(self) -> Box<TensorBase<S, D>> {
        Box::new(self)
    }

    pub fn context(&self) -> &Context {
        &self.ctx
    }

    pub const fn data(&self) -> &ArrayBase<S, D> {
        &self.data
    }

    pub fn dim(&self) -> D::Pattern {
        self.data.dim()
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
            ctx: self.ctx,
            data,
            op: self.op,
        })
    }

    pub fn into_dyn(self) -> TensorBase<S, IxDyn> {
        new!(self.data.into_dyn(), self.op.0)
    }

    pub fn into_owned(self) -> TensorBase<OwnedRepr<A>, D>
    where
        A: Clone,
        S: DataOwned,
    {
        new!(self.data.into_owned(), self.op.into_owned().0)
    }

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

    pub fn into_shared(self) -> TensorBase<OwnedArcRepr<A>, D>
    where
        S: DataOwned,
    {
        new!(self.data.into_shared(), self.op.into_shared().0)
    }

    pub fn is_variable(&self) -> bool {
        self.context().is_variable()
    }

    pub fn iter(&self) -> Iter<'_, A, D>
    where
        S: Data,
    {
        self.data().iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, A, D>
    where
        S: ndarray::DataMut,
    {
        self.data.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.data().len()
    }

    pub fn ndim(&self) -> usize {
        self.data().ndim()
    }

    pub fn raw_dim(&self) -> D {
        self.data().raw_dim()
    }

    pub fn shape(&self) -> &[usize] {
        self.data().shape()
    }

    pub fn strides(&self) -> &[isize] {
        self.data().strides()
    }

    pub fn to_owned(&self) -> crate::Tensor<A, D>
    where
        A: Clone,
        S: DataOwned + RawDataClone,
    {
        self.clone().into_owned()
    }

    pub fn to_shared(&self) -> crate::ArcTensor<A, D>
    where
        S: DataOwned + RawDataClone,
    {
        self.clone().into_shared()
    }

    pub fn view(&self) -> crate::TensorView<'_, A, D>
    where
        S: Data,
    {
        TensorBase {
            id: self.id,
            ctx: self.ctx,
            data: self.data.view(),
            op: self.op.view(),
        }
    }

    /// Gets an immutable reference to the operations of the tensor.
    pub fn op(&self) -> Option<&TensorExpr<S>> {
        self.op.as_ref()
    }

    pub fn variable(mut self) -> Self {
        self.ctx = self.ctx.into_var();
        self
    }

    pub fn with_ctx(mut self, ctx: Context) -> Self {
        self.ctx = ctx;
        self
    }

    pub fn with_op(mut self, op: impl Into<TensorOp<S>>) -> Self {
        self.op = op.into();
        self
    }
}

impl<'a, A, D> crate::CowTensor<'a, A, D>
where
    D: Dimension,
{
    pub fn is_view(&self) -> bool {
        self.data().is_view()
    }
}

impl<'a, A, D> crate::TensorView<'a, A, D>
where
    D: Dimension,
{
    pub fn reborrow<'b>(&'b self) -> crate::TensorView<'b, A, D> {
        // crate::TensorView {
        //     id: self.id,
        //     ctx: self.ctx,
        //     data: self.data.reborrow(),
        //     op: self.op.reborrow(),
        // }
        unimplemented!()
    }
}

impl<A, D> TensorBase<RawViewRepr<*const A>, D> where D: Dimension {
    pub unsafe fn cast<B>(self) -> crate::RawTensorView<B, D> where {
        TensorBase {
            id: self.id,
            ctx: self.ctx,
            data: self.data.cast::<B>(),
            op: self.op.cast(),
        }
    }
}

impl<A, S> TensorBase<S, ndarray::Ix0>
where
    S: RawData<Elem = A>,
{
    pub fn from_scalar(scalar: A) -> Self
    where
        A: Clone,
        S: DataOwned,
    {
        new!(ArrayBase::from_elem((), scalar))
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
        new!(data.into_dyn(), None)
    }
}

impl<S, D> Borrow<ArrayBase<S, D>> for TensorBase<S, D>
where
    D: Dimension,
    S: RawData,
{
    fn borrow(&self) -> &ArrayBase<S, D> {
        &self.data
    }
}

impl<S, D> BorrowMut<ArrayBase<S, D>> for TensorBase<S, D>
where
    D: Dimension,
    S: RawData,
{
    fn borrow_mut(&mut self) -> &mut ArrayBase<S, D> {
        &mut self.data
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
            ctx: self.ctx,
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

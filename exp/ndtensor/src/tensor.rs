/*
    Appellation: tensor <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{TensorError, TensorExpr, TensorId, TensorOp};
use crate::Context;
use core::borrow::{Borrow, BorrowMut};
use core::fmt;
use ndarray::*;

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
    /// Get an immutable reference to the [context](Context) of the tensor.
    pub fn ctx(&self) -> &Context {
        &self.ctx
    }
    /// Get a mutable reference to the [context](Context) of the tensor.
    pub fn ctx_mut(&mut self) -> &mut Context {
        &mut self.ctx
    }

    pub const fn data(&self) -> &ArrayBase<S, D> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut ArrayBase<S, D> {
        &mut self.data
    }

    pub fn dim(&self) -> D::Pattern {
        self.data.dim()
    }

    /// Returns the unique identifier of the tensor.
    pub const fn id(&self) -> TensorId {
        self.id
    }

    pub fn into_dimensionality<D2>(self) -> Result<TensorBase<S, D2>, TensorError>
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
        TensorBase {
            id: self.id,
            ctx: self.ctx,
            data: self.data.into_dyn(),
            op: self.op,
        }
    }

    pub fn to_dyn(&self) -> TensorBase<S, IxDyn>
    where
        S: RawDataClone,
    {
        TensorBase {
            id: self.id,
            ctx: self.ctx,
            data: self.data().clone().into_dyn(),
            op: self.op.clone(),
        }
    }

    pub fn into_owned(self) -> crate::Tensor<A, D>
    where
        A: Clone,
        S: DataOwned,
    {
        TensorBase {
            id: self.id,
            ctx: self.ctx,
            data: self.data.into_owned(),
            op: self.op.into_owned(),
        }
    }

    pub fn into_shared(self) -> crate::ArcTensor<A, D>
    where
        S: DataOwned,
    {
        TensorBase {
            id: self.id,
            ctx: self.ctx,
            data: self.data.into_shared(),
            op: self.op.into_shared(),
        }
    }

    pub fn is_variable(&self) -> bool {
        self.ctx().is_variable()
    }

    pub fn iter(&self) -> iter::Iter<'_, A, D>
    where
        S: Data,
    {
        self.data().iter()
    }

    pub fn iter_mut(&mut self) -> iter::IterMut<'_, A, D>
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

    pub fn raw_view(&self) -> crate::RawTensorView<A, D> {
        TensorBase {
            id: self.id,
            ctx: self.ctx,
            data: self.data.raw_view(),
            op: self.op.raw_view(),
        }
    }

    pub fn raw_view_mut(&mut self) -> crate::RawTensorViewMut<A, D>
    where
        S: RawDataMut,
    {
        TensorBase {
            id: self.id,
            ctx: self.ctx,
            data: self.data.raw_view_mut(),
            op: self.op.raw_view_mut(),
        }
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
        S: Data,
    {
        TensorBase {
            id: self.id,
            ctx: self.ctx,
            data: self.data.to_owned(),
            op: self.op.to_owned(),
        }
    }

    pub fn to_shared(&self) -> crate::ArcTensor<A, D>
    where
        A: Clone,
        S: Data,
    {
        TensorBase {
            id: self.id,
            ctx: self.ctx,
            data: self.data.to_shared(),
            op: self.op.to_shared(),
        }
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

    pub fn view_mut(&mut self) -> crate::TensorViewMut<'_, A, D>
    where
        S: DataMut,
    {
        TensorBase {
            id: self.id,
            ctx: self.ctx,
            data: self.data.view_mut(),
            op: self.op.view_mut(),
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

impl<A, D> crate::RawTensorView<A, D>
where
    D: Dimension,
{
    pub unsafe fn cast<B>(self) -> crate::RawTensorView<B, D> where {
        TensorBase {
            id: self.id,
            ctx: self.ctx,
            data: self.data.cast::<B>(),
            op: self.op.cast(),
        }
    }

    pub unsafe fn deref_into_view<'a>(self) -> crate::TensorView<'a, A, D> {
        TensorBase {
            id: self.id,
            ctx: self.ctx,
            data: self.data.deref_into_view(),
            op: self.op.deref_into_view(),
        }
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

// impl<S, D> Copy for TensorBase<S, D>
// where
//     D: Copy + Dimension,
//     S: Copy + RawDataClone,
// {
//     fn copy(&self) -> Self {
//         TensorBase {
//             id: self.id,
//             ctx: self.ctx,
//             data: self.data,
//             op: self.op,
//         }
//     }
// }

macro_rules! impl_fmt {
    ($(($trait:ident, $($fmt:tt)*)),*) => {
        impl_fmt!(@impl $(($trait, $($fmt)*)),*);
    };
    ($trait:ident, $($fmt:tt)*) => {
        impl_fmt!(@impl $trait, $($fmt)*);
    };
    (@impl $(($trait:ident, $($fmt:tt)*)),*) => {
        $(
            impl_fmt!(@impl $trait, $($fmt)*);
        )*
    };
    (@impl $trait:ident, $($fmt:tt)*) => {
        impl<A, S, D> core::fmt::$trait for TensorBase<S, D>
        where
            A: core::fmt::$trait,
            D: Dimension,
            S: Data<Elem = A>,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, $($fmt)*, self.data())
            }
        }
    };
}

impl_fmt!((Binary, "{:b}"), (Debug, "{:?}"), (Display, "{}"));

impl<A, S, D> PartialEq for TensorBase<S, D>
where
    D: Dimension,
    S: Data<Elem = A>,
    A: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<A, S, D> PartialEq<ArrayBase<S, D>> for TensorBase<S, D>
where
    D: Dimension,
    S: Data<Elem = A>,
    A: PartialEq,
{
    fn eq(&self, other: &ArrayBase<S, D>) -> bool {
        self.data == other
    }
}

impl<S, D, I> core::ops::Index<I> for TensorBase<S, D>
where
    D: Dimension,
    I: NdIndex<D>,
    S: Data,
{
    type Output = <S as RawData>::Elem;

    fn index(&self, index: I) -> &Self::Output {
        &self.data[index]
    }
}

impl<S, D, I> core::ops::IndexMut<I> for TensorBase<S, D>
where
    D: Dimension,
    I: NdIndex<D>,
    S: DataMut,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.data[index]
    }
}

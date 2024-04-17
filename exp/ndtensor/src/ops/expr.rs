/*
    Appellation: expr <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use crate::TensorBase;

use acme::ops::{BinaryOp, UnaryOp};
use ndarray::{
    Data, DataMut, DataOwned, OwnedArcRepr, OwnedRepr, RawData, RawDataClone, RawDataMut, ViewRepr,
};

pub type BoxTensor<S> = Box<TensorBase<S>>;

macro_rules! fwd_view_body {
    ($self:ident, $method:ident) => {
        match $self {
            TensorExpr::Binary { lhs, rhs, op } => TensorExpr::Binary {
                lhs: lhs.$method().boxed(),
                rhs: rhs.$method().boxed(),
                op,
            },
            TensorExpr::Unary { recv, op } => TensorExpr::Unary {
                recv: recv.$method().boxed(),
                op,
            },
            TensorExpr::Transpose(recv) => TensorExpr::Transpose(recv.$method().boxed()),
        }
    };
    (&$self:ident, $method:ident) => {
        match $self {
            TensorExpr::Binary { lhs, rhs, op } => TensorExpr::Binary {
                lhs: lhs.as_ref().$method().boxed(),
                rhs: rhs.as_ref().$method().boxed(),
                op: *op,
            },
            TensorExpr::Unary { recv, op } => TensorExpr::Unary {
                recv: recv.as_ref().$method().boxed(),
                op: *op,
            },
            TensorExpr::Transpose(recv) => TensorExpr::Transpose(recv.as_ref().$method().boxed()),
        }
    };
    (&mut $self:ident, $method:ident) => {
        match $self {
            TensorExpr::Binary { lhs, rhs, op } => TensorExpr::Binary {
                lhs: lhs.as_mut().$method().boxed(),
                rhs: rhs.as_mut().$method().boxed(),
                op: *op,
            },
            TensorExpr::Unary { recv, op } => TensorExpr::Unary {
                recv: recv.as_mut().$method().boxed(),
                op: *op,
            },
            TensorExpr::Transpose(recv) => TensorExpr::Transpose(recv.as_mut().$method().boxed()),
        }
    };
}

pub enum ReshapeExpr<S>
where
    S: RawData,
{
    Reshape(BoxTensor<S>, Vec<usize>),
    Transpose(BoxTensor<S>),
}

pub enum TensorExpr<S1, S2 = S1>
where
    S1: RawData,
    S2: RawData,
{
    Binary {
        lhs: BoxTensor<S1>,
        rhs: BoxTensor<S2>,
        op: BinaryOp,
    },
    Unary {
        recv: BoxTensor<S1>,
        op: UnaryOp,
    },
    Transpose(BoxTensor<S1>),
}

impl<A, B, S1, S2> TensorExpr<S1, S2>
where
    S1: RawData<Elem = A>,
    S2: RawData<Elem = B>,
{
    pub fn binary(lhs: BoxTensor<S1>, rhs: BoxTensor<S2>, op: BinaryOp) -> Self {
        TensorExpr::Binary { lhs, rhs, op }
    }

    pub fn transpose(recv: BoxTensor<S1>) -> Self {
        TensorExpr::Transpose(recv)
    }

    pub fn unary(recv: BoxTensor<S1>, op: UnaryOp) -> Self {
        TensorExpr::Unary { recv, op }
    }

    pub fn into_owned(self) -> TensorExpr<OwnedRepr<A>, OwnedRepr<B>>
    where
        A: Clone,
        B: Clone,
        S1: DataOwned,
        S2: DataOwned,
    {
        fwd_view_body!(self, into_owned)
    }

    pub fn into_shared(self) -> TensorExpr<OwnedArcRepr<A>, OwnedArcRepr<B>>
    where
        S1: DataOwned,
        S2: DataOwned,
    {
        fwd_view_body!(self, into_shared)
    }

    pub fn raw_view(&self) -> TensorExpr<RawViewRepr<*const A>, RawViewRepr<*const B>> {
        fwd_view_body!(&self, raw_view)
    }

    pub fn raw_view_mut(&mut self) -> TensorExpr<RawViewRepr<*mut A>, RawViewRepr<*mut B>>
    where
        S1: RawDataMut,
        S2: RawDataMut,
    {
        fwd_view_body!(&mut self, raw_view_mut)
    }

    pub fn to_owned(&self) -> TensorExpr<OwnedRepr<A>, OwnedRepr<B>>
    where
        A: Clone,
        B: Clone,
        S1: Data,
        S2: Data,
    {
        fwd_view_body!(&self, to_owned)
    }

    pub fn to_shared(&self) -> TensorExpr<OwnedArcRepr<A>, OwnedArcRepr<B>>
    where
        A: Clone,
        B: Clone,
        S1: Data,
        S2: Data,
    {
        fwd_view_body!(&self, to_shared)
    }

    pub fn view(&self) -> TensorExpr<ViewRepr<&'_ A>, ViewRepr<&'_ B>>
    where
        S1: Data,
        S2: Data,
    {
        fwd_view_body!(&self, view)
    }

    pub fn view_mut(&mut self) -> TensorExpr<ViewRepr<&'_ mut A>, ViewRepr<&'_ mut B>>
    where
        S1: DataMut,
        S2: DataMut,
    {
        fwd_view_body!(&mut self, view_mut)
    }
}

use ndarray::RawViewRepr;

impl<A, B> TensorExpr<RawViewRepr<*const A>, RawViewRepr<*const B>> {
    pub unsafe fn cast<C>(self) -> TensorExpr<RawViewRepr<*const C>, RawViewRepr<*const C>> where {
        match self {
            TensorExpr::Binary { lhs, rhs, op } => TensorExpr::Binary {
                lhs: lhs.cast().boxed(),
                rhs: rhs.cast().boxed(),
                op,
            },
            TensorExpr::Unary { recv, op } => TensorExpr::Unary {
                recv: recv.cast().boxed(),
                op,
            },
            TensorExpr::Transpose(recv) => TensorExpr::Transpose(recv.cast().boxed()),
        }
    }

    pub unsafe fn deref_into_view<'a>(self) -> TensorExpr<ViewRepr<&'a A>, ViewRepr<&'a B>> where {
        match self {
            TensorExpr::Binary { lhs, rhs, op } => TensorExpr::Binary {
                lhs: lhs.deref_into_view().boxed(),
                rhs: rhs.deref_into_view().boxed(),
                op,
            },
            TensorExpr::Unary { recv, op } => TensorExpr::Unary {
                recv: recv.deref_into_view().boxed(),
                op,
            },
            TensorExpr::Transpose(recv) => TensorExpr::Transpose(recv.deref_into_view().boxed()),
        }
    }
}

impl<S1, S2> Clone for TensorExpr<S1, S2>
where
    S1: RawDataClone,
    S2: RawDataClone,
{
    fn clone(&self) -> Self {
        match self {
            TensorExpr::Binary { lhs, rhs, op } => TensorExpr::Binary {
                lhs: lhs.clone(),
                rhs: rhs.clone(),
                op: *op,
            },
            TensorExpr::Unary { recv, op } => TensorExpr::Unary {
                recv: recv.clone(),
                op: *op,
            },
            TensorExpr::Transpose(recv) => TensorExpr::Transpose(recv.clone()),
        }
    }
}

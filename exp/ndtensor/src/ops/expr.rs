/*
    Appellation: expr <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::nd::*;
use crate::TensorBase;

use acme::ops::{BinaryOp, UnaryOp};

pub type BoxTensor<S> = Box<TensorBase<S>>;

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
}

impl<A, B, S1, S2> TensorExpr<S1, S2>
where
    S1: RawData<Elem = A>,
    S2: RawData<Elem = B>,
{
    pub fn binary(lhs: BoxTensor<S1>, rhs: BoxTensor<S2>, op: BinaryOp) -> Self {
        TensorExpr::Binary { lhs, rhs, op }
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
        match self {
            TensorExpr::Binary { lhs, rhs, op } => TensorExpr::Binary {
                lhs: lhs.into_owned().boxed(),
                rhs: rhs.into_owned().boxed(),
                op,
            },
            TensorExpr::Unary { recv, op } => TensorExpr::Unary {
                recv: recv.into_owned().boxed(),
                op,
            },
        }
    }

    pub fn into_shared(self) -> TensorExpr<OwnedArcRepr<A>, OwnedArcRepr<B>>
    where
        S1: DataOwned,
        S2: DataOwned,
    {
        match self {
            TensorExpr::Binary { lhs, rhs, op } => TensorExpr::Binary {
                lhs: lhs.into_shared().boxed(),
                rhs: rhs.into_shared().boxed(),
                op,
            },
            TensorExpr::Unary { recv, op } => TensorExpr::Unary {
                recv: recv.into_shared().boxed(),
                op,
            },
        }
    }

    pub fn raw_view(&self) -> TensorExpr<RawViewRepr<*const A>, RawViewRepr<*const B>> {
        match self {
            TensorExpr::Binary { lhs, rhs, op } => TensorExpr::Binary {
                lhs: lhs.raw_view().boxed(),
                rhs: rhs.raw_view().boxed(),
                op: *op,
            },
            TensorExpr::Unary { recv, op } => TensorExpr::Unary {
                recv: recv.raw_view().boxed(),
                op: *op,
            },
        }
    }

    pub fn raw_view_mut(&mut self) -> TensorExpr<RawViewRepr<*mut A>, RawViewRepr<*mut B>>
    where
        S1: RawDataMut,
        S2: RawDataMut,
    {
        match self {
            TensorExpr::Binary { lhs, rhs, op } => TensorExpr::Binary {
                lhs: lhs.raw_view_mut().boxed(),
                rhs: rhs.raw_view_mut().boxed(),
                op: *op,
            },
            TensorExpr::Unary { recv, op } => TensorExpr::Unary {
                recv: recv.raw_view_mut().boxed(),
                op: *op,
            },
        }
    }

    pub fn to_owned(&self) -> TensorExpr<OwnedRepr<A>, OwnedRepr<B>>
    where
        A: Clone,
        B: Clone,
        S1: Data,
        S2: Data,
    {
        match self {
            TensorExpr::Binary { lhs, rhs, op } => TensorExpr::Binary {
                lhs: lhs.as_ref().to_owned().boxed(),
                rhs: rhs.as_ref().to_owned().boxed(),
                op: *op,
            },
            TensorExpr::Unary { recv, op } => TensorExpr::Unary {
                recv: recv.as_ref().to_owned().boxed(),
                op: *op,
            },
        }
    }

    pub fn to_shared(&self) -> TensorExpr<OwnedArcRepr<A>, OwnedArcRepr<B>>
    where
        A: Clone,
        B: Clone,
        S1: Data,
        S2: Data,
    {
        match self {
            TensorExpr::Binary { lhs, rhs, op } => TensorExpr::Binary {
                lhs: lhs.to_shared().boxed(),
                rhs: rhs.to_shared().boxed(),
                op: *op,
            },
            TensorExpr::Unary { recv, op } => TensorExpr::Unary {
                recv: recv.to_shared().boxed(),
                op: *op,
            },
        }
    }

    pub fn view(&self) -> TensorExpr<ViewRepr<&'_ A>, ViewRepr<&'_ B>>
    where
        S1: Data,
        S2: Data,
    {
        match self {
            TensorExpr::Binary { lhs, rhs, op } => TensorExpr::Binary {
                lhs: lhs.view().boxed(),
                rhs: rhs.view().boxed(),
                op: *op,
            },
            TensorExpr::Unary { recv, op } => TensorExpr::Unary {
                recv: recv.view().boxed(),
                op: *op,
            },
        }
    }

    pub fn view_mut(&mut self) -> TensorExpr<ViewRepr<&'_ mut A>, ViewRepr<&'_ mut B>>
    where
        S1: DataMut,
        S2: DataMut,
    {
        match self {
            TensorExpr::Binary { lhs, rhs, op } => TensorExpr::Binary {
                lhs: lhs.view_mut().boxed(),
                rhs: rhs.view_mut().boxed(),
                op: *op,
            },
            TensorExpr::Unary { recv, op } => TensorExpr::Unary {
                recv: recv.view_mut().boxed(),
                op: *op,
            },
        }
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
        }
    }
}

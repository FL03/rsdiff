/*
    Appellation: expr <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::TensorBase;
use acme::ops::{BinaryOp, UnaryOp};
use ndarray::{DataOwned, OwnedArcRepr, OwnedRepr, RawData, RawDataClone};

pub type BoxTensor<S> = Box<TensorBase<S>>;

pub enum TensorExpr<S>
where
    S: RawData,
{
    Binary {
        lhs: BoxTensor<S>,
        rhs: BoxTensor<S>,
        op: BinaryOp,
    },
    Unary {
        recv: BoxTensor<S>,
        op: UnaryOp,
    },
}

impl<A, S> TensorExpr<S>
where
    S: RawData<Elem = A>,
{
    pub fn binary(lhs: BoxTensor<S>, rhs: BoxTensor<S>, op: BinaryOp) -> Self {
        TensorExpr::Binary { lhs, rhs, op }
    }

    pub fn unary(recv: BoxTensor<S>, op: UnaryOp) -> Self {
        TensorExpr::Unary { recv, op }
    }

    pub fn into_owned(self) -> TensorExpr<OwnedRepr<A>>
    where
        A: Clone,
        S: DataOwned,
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

    pub fn into_shared(self) -> TensorExpr<OwnedArcRepr<A>>
    where
        S: DataOwned,
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
}

impl<S> Clone for TensorExpr<S>
where
    S: RawDataClone,
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

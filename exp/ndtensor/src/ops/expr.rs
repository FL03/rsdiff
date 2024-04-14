/*
    Appellation: expr <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::TensorBase;
use acme::ops::{BinaryOp, UnaryOp};
use ndarray::{DataOwned, OwnedArcRepr, OwnedRepr, RawData, RawDataClone};

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

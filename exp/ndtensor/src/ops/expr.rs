/*
    Appellation: expr <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::TensorBase;
use acme::ops::{BinaryOp, UnaryOp};
use ndarray::RawData;

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

impl<S> TensorExpr<S>
where
    S: RawData,
{
    pub fn binary(lhs: BoxTensor<S>, rhs: BoxTensor<S>, op: BinaryOp) -> Self {
        TensorExpr::Binary { lhs, rhs, op }
    }

    pub fn unary(recv: BoxTensor<S>, op: UnaryOp) -> Self {
        TensorExpr::Unary { recv, op }
    }
}

impl<S> Clone for TensorExpr<S>
where
    S: ndarray::RawDataClone,
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

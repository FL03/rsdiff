/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::TensorBase;
use acme::ops::binary::BinaryOp;
use acme::ops::unary::UnaryOp;

#[derive(Clone, Debug)]
pub enum TensorOp<T> {
    Binary(Box<TensorBase<T>>, Box<TensorBase<T>>, BinaryOp),
    BinaryScalar(Box<TensorBase<T>>, T, BinaryOp),
    Unary(Box<TensorBase<T>>, UnaryOp),
    Matmul(Box<TensorBase<T>>, Box<TensorBase<T>>),
}

impl<T> TensorOp<T> {
    pub fn binary(lhs: TensorBase<T>, rhs: TensorBase<T>, op: BinaryOp) -> Self {
        TensorOp::Binary(Box::new(lhs), Box::new(rhs), op)
    }

    pub fn binary_scalar(lhs: TensorBase<T>, rhs: T, op: BinaryOp) -> Self {
        TensorOp::BinaryScalar(Box::new(lhs), rhs, op)
    }

    pub fn unary(tensor: TensorBase<T>, op: UnaryOp) -> Self {
        TensorOp::Unary(Box::new(tensor), op)
    }

    pub fn matmul(lhs: TensorBase<T>, rhs: TensorBase<T>) -> Self {
        TensorOp::Matmul(Box::new(lhs), Box::new(rhs))
    }

    pub fn lhs(&self) -> &TensorBase<T> {
        match self {
            TensorOp::Binary(lhs, _, _) => lhs,
            TensorOp::BinaryScalar(lhs, _, _) => lhs,
            TensorOp::Unary(lhs, _) => lhs,
            TensorOp::Matmul(lhs, _) => lhs,
        }
    }

    pub fn rhs(&self) -> Option<&TensorBase<T>> {
        match self {
            TensorOp::Binary(_, rhs, _) => Some(rhs),
            TensorOp::Matmul(_, rhs) => Some(rhs),
            _ => None,
        }
    }
}

pub enum Inputs<T> {
    Scalar(T),
    Tensor(TensorBase<T>),
}

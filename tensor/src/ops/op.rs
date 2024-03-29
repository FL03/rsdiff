/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::kinds::reshape::*;
use crate::shape::{Axis, Shape};
use crate::TensorBase;
use acme::prelude::{BinaryOp, UnaryOp};

pub type BoxTensor<T = f64> = Box<TensorBase<T>>;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum TensorOp<T> {
    Binary(BoxTensor<T>, BoxTensor<T>, BinaryOp),
    BinaryScalar(BoxTensor<T>, T, BinaryOp),
    Unary(BoxTensor<T>, UnaryOp),
    Broadcast(BoxTensor<T>, Shape),
    Matmul(BoxTensor<T>, BoxTensor<T>),
    Reshape(ReshapeExpr<T>),
    Transpose {
        scope: BoxTensor<T>,
        target: (Axis, Axis),
    },
}

impl<T> TensorOp<T> {
    pub fn binary(lhs: TensorBase<T>, rhs: TensorBase<T>, op: BinaryOp) -> Self {
        TensorOp::Binary(Box::new(lhs), Box::new(rhs), op)
    }

    pub fn binary_scalar(lhs: TensorBase<T>, rhs: T, op: BinaryOp) -> Self {
        TensorOp::BinaryScalar(Box::new(lhs), rhs, op)
    }

    pub fn broadcast(tensor: TensorBase<T>, shape: Shape) -> Self {
        TensorOp::Broadcast(Box::new(tensor), shape)
    }

    pub fn matmul(lhs: TensorBase<T>, rhs: TensorBase<T>) -> Self {
        TensorOp::Matmul(Box::new(lhs), Box::new(rhs))
    }

    pub fn transpose(scope: TensorBase<T>, swap: Axis, with: Axis) -> Self {
        TensorOp::Transpose {
            scope: Box::new(scope),
            target: (swap, with),
        }
    }

    pub fn unary(tensor: TensorBase<T>, op: UnaryOp) -> Self {
        TensorOp::Unary(Box::new(tensor), op)
    }
}
impl<T> TensorOp<T> {
    pub fn lhs(self) -> Option<TensorBase<T>> {
        match self {
            TensorOp::Binary(lhs, _, _) => Some(*lhs),
            TensorOp::BinaryScalar(lhs, _, _) => Some(*lhs),
            TensorOp::Unary(lhs, _) => Some(*lhs),
            TensorOp::Broadcast(tensor, _) => Some(*tensor),
            TensorOp::Matmul(lhs, _) => Some(*lhs),
            TensorOp::Transpose { scope, .. } => Some(*scope),
            _ => None,
        }
    }

    pub fn rhs(self) -> Option<TensorBase<T>> {
        match self {
            TensorOp::Binary(_, rhs, _) => Some(*rhs),
            TensorOp::BinaryScalar(_, scalar, _) => Some(TensorBase::from_scalar(scalar)),
            TensorOp::Matmul(_, rhs) => Some(*rhs),
            _ => None,
        }
    }
}

impl<T> TensorOp<T>
where
    T: Clone,
{
    pub fn view<'a>(&'a self) -> TensorOp<&'a T> {
        match self {
            TensorOp::Binary(lhs, rhs, op) => TensorOp::binary(lhs.view(), rhs.view(), *op),
            TensorOp::BinaryScalar(lhs, rhs, op) => TensorOp::binary_scalar(lhs.view(), rhs, *op),
            TensorOp::Unary(tensor, op) => TensorOp::unary(tensor.view(), *op),
            TensorOp::Broadcast(tensor, shape) => TensorOp::broadcast(tensor.view(), shape.clone()),
            TensorOp::Matmul(lhs, rhs) => TensorOp::matmul(lhs.view(), rhs.view()),
            TensorOp::Transpose {
                scope: tensor,
                target: axes,
            } => TensorOp::transpose(tensor.view(), axes.0, axes.1),
            _ => unimplemented!()
        }
    }
}

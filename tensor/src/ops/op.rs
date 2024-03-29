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
pub enum TensorExpr<T> {
    Binary(BoxTensor<T>, BoxTensor<T>, BinaryOp),
    BinaryScalar(BoxTensor<T>, T, BinaryOp),
    Unary(BoxTensor<T>, UnaryOp),
    Broadcast(BoxTensor<T>, Shape),
    Matmul(BoxTensor<T>, BoxTensor<T>),
    Reshape(BoxTensor<T>, ReshapeExpr<T>),
    Shape(ReshapeExpr<T>),
    Transpose {
        scope: BoxTensor<T>,
        target: (Axis, Axis),
    },
}

impl<T> TensorExpr<T> {
    pub fn binary(lhs: TensorBase<T>, rhs: TensorBase<T>, op: BinaryOp) -> Self {
        TensorExpr::Binary(Box::new(lhs), Box::new(rhs), op)
    }

    pub fn binary_scalar(lhs: TensorBase<T>, rhs: T, op: BinaryOp) -> Self {
        TensorExpr::BinaryScalar(Box::new(lhs), rhs, op)
    }

    pub fn broadcast(tensor: TensorBase<T>, shape: Shape) -> Self {
        TensorExpr::Broadcast(Box::new(tensor), shape)
    }

    pub fn matmul(lhs: TensorBase<T>, rhs: TensorBase<T>) -> Self {
        TensorExpr::Matmul(Box::new(lhs), Box::new(rhs))
    }

    pub fn transpose(scope: TensorBase<T>, swap: Axis, with: Axis) -> Self {
        TensorExpr::Transpose {
            scope: Box::new(scope),
            target: (swap, with),
        }
    }

    pub fn unary(tensor: TensorBase<T>, op: UnaryOp) -> Self {
        TensorExpr::Unary(Box::new(tensor), op)
    }
}
impl<T> TensorExpr<T> {
    pub fn lhs(self) -> Option<TensorBase<T>> {
        match self {
            TensorExpr::Binary(lhs, _, _) => Some(*lhs),
            TensorExpr::BinaryScalar(lhs, _, _) => Some(*lhs),
            TensorExpr::Unary(lhs, _) => Some(*lhs),
            TensorExpr::Broadcast(tensor, _) => Some(*tensor),
            TensorExpr::Matmul(lhs, _) => Some(*lhs),
            TensorExpr::Transpose { scope, .. } => Some(*scope),
            _ => None,
        }
    }

    pub fn rhs(self) -> Option<TensorBase<T>> {
        match self {
            TensorExpr::Binary(_, rhs, _) => Some(*rhs),
            TensorExpr::BinaryScalar(_, scalar, _) => Some(TensorBase::from_scalar(scalar)),
            TensorExpr::Matmul(_, rhs) => Some(*rhs),
            _ => None,
        }
    }
}

impl<T> TensorExpr<T>
where
    T: Clone,
{
    pub fn view<'a>(&'a self) -> TensorExpr<&'a T> {
        match self {
            TensorExpr::Binary(lhs, rhs, op) => TensorExpr::binary(lhs.view(), rhs.view(), *op),
            TensorExpr::BinaryScalar(lhs, rhs, op) => {
                TensorExpr::binary_scalar(lhs.view(), rhs, *op)
            }
            TensorExpr::Unary(tensor, op) => TensorExpr::unary(tensor.view(), *op),
            TensorExpr::Broadcast(tensor, shape) => {
                TensorExpr::broadcast(tensor.view(), shape.clone())
            }
            TensorExpr::Matmul(lhs, rhs) => TensorExpr::matmul(lhs.view(), rhs.view()),
            TensorExpr::Transpose {
                scope: tensor,
                target: axes,
            } => TensorExpr::transpose(tensor.view(), axes.0, axes.1),
            _ => unimplemented!(),
        }
    }
}

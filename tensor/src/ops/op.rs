/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::kinds::reshape::*;
use crate::shape::{Axis, Shape};
use crate::TensorBase;
use acme::prelude::{BinaryOp, UnaryOp};
use num::Complex;

pub type BoxTensor<T = f64> = Box<TensorBase<T>>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum TensorExpr<A, B = A> {
    Binary(BoxTensor<A>, BoxTensor<B>, BinaryOp),
    BinaryScalar(BoxTensor<A>, B, BinaryOp),
    Unary(BoxTensor<A>, UnaryOp),
    Broadcast(BoxTensor<A>, Shape),
    Matmul(BoxTensor<A>, BoxTensor<B>),
    Reshape(BoxTensor<A>, Shape),
    Shape(ReshapeExpr<A>),
    SwapAxes(BoxTensor<A>, Axis, Axis),
    Transpose(BoxTensor<A>),
}

impl<A, B> TensorExpr<A, B> {
    pub fn binary(lhs: TensorBase<A>, rhs: TensorBase<B>, op: BinaryOp) -> Self {
        TensorExpr::Binary(Box::new(lhs), Box::new(rhs), op)
    }

    pub fn binary_scalar(lhs: TensorBase<A>, rhs: B, op: BinaryOp) -> Self {
        TensorExpr::BinaryScalar(Box::new(lhs), rhs, op)
    }

    pub fn binary_scalar_c(
        lhs: TensorBase<A>,
        rhs: Complex<A>,
        op: BinaryOp,
    ) -> TensorExpr<A, Complex<A>> {
        TensorExpr::BinaryScalar(Box::new(lhs), rhs, op)
    }

    pub fn broadcast(tensor: TensorBase<A>, shape: Shape) -> Self {
        TensorExpr::Broadcast(Box::new(tensor), shape)
    }

    pub fn matmul(lhs: TensorBase<A>, rhs: TensorBase<B>) -> Self {
        TensorExpr::Matmul(Box::new(lhs), Box::new(rhs))
    }

    pub fn reshape(tensor: TensorBase<A>, shape: Shape) -> Self {
        TensorExpr::Reshape(Box::new(tensor), shape)
    }

    pub fn shape(expr: ReshapeExpr<A>) -> Self {
        TensorExpr::Shape(expr)
    }

    pub fn swap_axes(tensor: TensorBase<A>, swap: Axis, with: Axis) -> Self {
        TensorExpr::SwapAxes(Box::new(tensor), swap, with)
    }

    pub fn transpose(scope: TensorBase<A>) -> Self {
        TensorExpr::Transpose(Box::new(scope))
    }

    pub fn unary(tensor: TensorBase<A>, op: UnaryOp) -> Self {
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
            TensorExpr::Transpose(lhs) => Some(*lhs),
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
            TensorExpr::Reshape(tensor, shape) => TensorExpr::reshape(tensor.view(), shape.clone()),
            TensorExpr::Transpose(tensor) => TensorExpr::transpose(tensor.view()),
            _ => unimplemented!(),
        }
    }
}

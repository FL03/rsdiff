/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::kinds::reshape::*;
use crate::shape::{Axis, Shape};
use crate::tensor::TensorBase;
use acme::prelude::{BinaryOp, UnaryOp};
use num::Complex;

pub type BoxTensor<T = f64> = Box<TensorBase<T>>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[non_exhaustive]
pub enum TensorExpr<A, B = A> {
    Binary(BoxTensor<A>, BoxTensor<B>, BinaryOp),
    BinaryScalar(BoxTensor<A>, B, BinaryOp),
    Unary(BoxTensor<A>, UnaryOp),
    Matmul(BoxTensor<A>, BoxTensor<B>),
    Sigmoid(BoxTensor<A>),
    Shape(ReshapeExpr<A>),
}

impl<A, B> TensorExpr<A, B> {
    pub fn binary(lhs: TensorBase<A>, rhs: TensorBase<B>, op: BinaryOp) -> Self {
        Self::Binary(lhs.boxed(), rhs.boxed(), op)
    }

    pub fn binary_scalar(lhs: TensorBase<A>, rhs: B, op: BinaryOp) -> Self {
        Self::BinaryScalar(Box::new(lhs), rhs, op)
    }

    pub fn binary_scalar_c(
        lhs: TensorBase<A>,
        rhs: Complex<A>,
        op: BinaryOp,
    ) -> TensorExpr<A, Complex<A>> {
        TensorExpr::BinaryScalar(Box::new(lhs), rhs, op)
    }

    pub fn broadcast(tensor: TensorBase<A>, shape: Shape) -> Self {
        Self::shape(ReshapeExpr::broadcast(tensor, shape))
    }

    pub fn matmul(lhs: TensorBase<A>, rhs: TensorBase<B>) -> Self {
        Self::Matmul(Box::new(lhs), Box::new(rhs))
    }

    pub fn reshape(tensor: TensorBase<A>, shape: Shape) -> Self {
        Self::shape(ReshapeExpr::reshape(tensor, shape))
    }

    pub fn shape(expr: ReshapeExpr<A>) -> Self {
        Self::Shape(expr)
    }

    pub fn sigmoid(tensor: TensorBase<A>) -> Self {
        Self::Sigmoid(Box::new(tensor))
    }

    pub fn swap_axes(tensor: TensorBase<A>, swap: Axis, with: Axis) -> Self {
        Self::shape(ReshapeExpr::swap_axes(tensor, swap, with))
    }

    pub fn transpose(tensor: TensorBase<A>) -> Self {
        Self::Shape(ReshapeExpr::transpose(tensor))
    }

    pub fn unary(tensor: TensorBase<A>, op: UnaryOp) -> Self {
        Self::Unary(Box::new(tensor), op)
    }

    pub fn lhs(self) -> Option<TensorBase<A>> {
        match self {
            Self::Binary(lhs, _, _) => Some(*lhs),
            Self::BinaryScalar(lhs, _, _) => Some(*lhs),
            Self::Unary(lhs, _) => Some(*lhs),
            Self::Matmul(lhs, _) => Some(*lhs),
            _ => None,
        }
    }

    pub fn rhs(self) -> Option<TensorBase<B>> {
        match self {
            Self::Binary(_, rhs, _) => Some(*rhs),
            Self::BinaryScalar(_, scalar, _) => Some(TensorBase::from_scalar(scalar)),
            Self::Matmul(_, rhs) => Some(*rhs),
            _ => None,
        }
    }
    pub fn view(&self) -> TensorExpr<&A, &B> {
        match self {
            TensorExpr::Binary(lhs, rhs, op) => TensorExpr::binary(lhs.view(), rhs.view(), *op),
            TensorExpr::BinaryScalar(lhs, rhs, op) => {
                TensorExpr::binary_scalar(lhs.view(), rhs, *op)
            }
            TensorExpr::Unary(tensor, op) => TensorExpr::unary(tensor.view(), *op),
            TensorExpr::Matmul(lhs, rhs) => TensorExpr::matmul(lhs.view(), rhs.view()),
            TensorExpr::Sigmoid(tensor) => TensorExpr::sigmoid(tensor.view()),
            TensorExpr::Shape(inner) => TensorExpr::Shape(inner.view()),
        }
    }
    pub fn view_mut(&mut self) -> TensorExpr<&mut A, &mut B> {
        match self {
            TensorExpr::Binary(lhs, rhs, op) => {
                TensorExpr::binary(lhs.view_mut(), rhs.view_mut(), *op)
            }

            TensorExpr::BinaryScalar(lhs, rhs, op) => {
                TensorExpr::binary_scalar(lhs.view_mut(), rhs, *op)
            }
            TensorExpr::Unary(tensor, op) => TensorExpr::unary(tensor.view_mut(), *op),
            TensorExpr::Matmul(lhs, rhs) => TensorExpr::matmul(lhs.view_mut(), rhs.view_mut()),
            TensorExpr::Sigmoid(tensor) => TensorExpr::sigmoid(tensor.view_mut()),
            TensorExpr::Shape(inner) => TensorExpr::Shape(inner.view_mut()),
        }
    }
}

/*
    Appellation: backprop <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::TensorExpr;
use crate::TensorBase;
use acme::prelude::BinaryOp;
use core::borrow::Borrow;
use core::ops::{Deref, DerefMut};

pub trait TensorOp {
    type Output;

    fn name(&self) -> &str;
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct BackpropOp<A = f64, B = A>(Option<TensorExpr<A, B>>);

impl<A, B> BackpropOp<A, B> {
    pub fn new(op: TensorExpr<A, B>) -> Self {
        BackpropOp(Some(op))
    }

    pub fn none() -> Self {
        BackpropOp(None)
    }

    pub fn binary(lhs: TensorBase<A>, rhs: TensorBase<B>, kind: BinaryOp) -> Self {
        BackpropOp(Some(TensorExpr::binary(lhs, rhs, kind)))
    }

    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    pub fn op(&self) -> Option<&TensorExpr<A, B>> {
        self.0.as_ref()
    }

    pub fn op_mut(&mut self) -> Option<&mut TensorExpr<A, B>> {
        self.0.as_mut()
    }

    pub fn into_inner(self) -> Option<TensorExpr<A, B>> {
        self.0
    }

    pub fn take(&mut self) -> Option<TensorExpr<A, B>> {
        self.0.take()
    }

    pub fn view(&self) -> BackpropOp<&A, &B> {
        BackpropOp(self.0.as_ref().map(|op| op.view()))
    }

    pub fn view_mut(&mut self) -> BackpropOp<&mut A, &mut B> {
        BackpropOp(self.0.as_mut().map(|op| op.view_mut()))
    }
}

impl<S, T> Borrow<Option<TensorExpr<S, T>>> for BackpropOp<S, T> {
    fn borrow(&self) -> &Option<TensorExpr<S, T>> {
        &self.0
    }
}

impl<T> Default for BackpropOp<T> {
    fn default() -> Self {
        Self::none()
    }
}

impl<T> Deref for BackpropOp<T> {
    type Target = Option<TensorExpr<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for BackpropOp<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<Option<TensorExpr<T>>> for BackpropOp<T> {
    fn from(op: Option<TensorExpr<T>>) -> Self {
        BackpropOp(op)
    }
}

impl<T> From<TensorExpr<T>> for BackpropOp<T> {
    fn from(op: TensorExpr<T>) -> Self {
        BackpropOp(Some(op))
    }
}

impl<T> From<BackpropOp<T>> for Option<TensorExpr<T>> {
    fn from(op: BackpropOp<T>) -> Option<TensorExpr<T>> {
        op.into_inner()
    }
}

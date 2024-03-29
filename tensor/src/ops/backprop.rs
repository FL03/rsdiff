/*
    Appellation: backprop <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::TensorExpr;
use crate::TensorBase;
use acme::prelude::BinaryOp;
use core::borrow::Borrow;
use core::ops::{Deref, DerefMut};

#[derive(Clone, Debug)]
pub struct BackpropOp<T = f64>(Option<TensorExpr<T>>);

impl<T> BackpropOp<T> {
    pub fn new(op: TensorExpr<T>) -> Self {
        BackpropOp(Some(op))
    }

    pub fn none() -> Self {
        BackpropOp(None)
    }

    pub fn binary(lhs: TensorBase<T>, rhs: TensorBase<T>, kind: BinaryOp) -> Self {
        BackpropOp(Some(TensorExpr::binary(lhs, rhs, kind)))
    }

    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    pub fn op(&self) -> Option<&TensorExpr<T>> {
        self.0.as_ref()
    }

    pub fn op_mut(&mut self) -> Option<&mut TensorExpr<T>> {
        self.0.as_mut()
    }

    pub fn into_inner(self) -> Option<TensorExpr<T>> {
        self.0
    }

    pub fn take(&mut self) -> Option<TensorExpr<T>> {
        self.0.take()
    }
}

impl<T> BackpropOp<T>
where
    T: Clone,
{
    pub fn view(&self) -> BackpropOp<&T> {
        BackpropOp(self.0.as_ref().map(|op| op.view()))
    }
}

impl<T> Borrow<Option<TensorExpr<T>>> for BackpropOp<T> {
    fn borrow(&self) -> &Option<TensorExpr<T>> {
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

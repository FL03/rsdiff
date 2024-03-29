/*
    Appellation: backprop <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::TensorOp;
use crate::TensorBase;
use acme::prelude::BinaryOp;
use core::borrow::Borrow;
use core::ops::{Deref, DerefMut};

#[derive(Clone, Debug)]
pub struct BackpropOp<T>(Option<TensorOp<T>>);

impl<T> BackpropOp<T> {
    pub fn new(op: TensorOp<T>) -> Self {
        BackpropOp(Some(op))
    }

    pub fn none() -> Self {
        BackpropOp(None)
    }

    pub fn binary(lhs: TensorBase<T>, rhs: TensorBase<T>, kind: BinaryOp) -> Self {
        BackpropOp(Some(TensorOp::binary(lhs, rhs, kind)))
    }

    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    pub fn op(&self) -> Option<&TensorOp<T>> {
        self.0.as_ref()
    }

    pub fn op_mut(&mut self) -> Option<&mut TensorOp<T>> {
        self.0.as_mut()
    }

    pub fn into_inner(self) -> Option<TensorOp<T>> {
        self.0
    }

    pub fn take(&mut self) -> Option<TensorOp<T>> {
        self.0.take()
    }
}

impl<T> BackpropOp<T> where T: Clone {
    pub fn view(&self) -> BackpropOp<&T> {
        BackpropOp(self.0.as_ref().map(|op| op.view()))
    }
}

impl<T> Borrow<Option<TensorOp<T>>> for BackpropOp<T> {
    fn borrow(&self) -> &Option<TensorOp<T>> {
        &self.0
    }
}

impl<T> Default for BackpropOp<T> {
    fn default() -> Self {
        Self::none()
    }
}

impl<T> Deref for BackpropOp<T> {
    type Target = Option<TensorOp<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for BackpropOp<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<Option<TensorOp<T>>> for BackpropOp<T> {
    fn from(op: Option<TensorOp<T>>) -> Self {
        BackpropOp(op)
    }
}

impl<T> From<TensorOp<T>> for BackpropOp<T> {
    fn from(op: TensorOp<T>) -> Self {
        BackpropOp(Some(op))
    }
}

impl<T> From<BackpropOp<T>> for Option<TensorOp<T>> {
    fn from(op: BackpropOp<T>) -> Option<TensorOp<T>> {
        op.into_inner()
    }
}

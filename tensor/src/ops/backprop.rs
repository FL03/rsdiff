/*
    Appellation: backprop <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::TensorOp;

#[derive(Clone, Debug)]
pub struct TrackedOp<T>(Option<TensorOp<T>>);

impl<T> TrackedOp<T> {
    pub fn new(op: TensorOp<T>) -> Self {
        TrackedOp(Some(op))
    }

    pub fn none() -> Self {
        TrackedOp(None)
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
}

impl<T> Default for TrackedOp<T> {
    fn default() -> Self {
        Self::none()
    }
}

impl<T> From<Option<TensorOp<T>>> for TrackedOp<T> {
    fn from(op: Option<TensorOp<T>>) -> Self {
        TrackedOp(op)
    }
}

impl<T> From<TensorOp<T>> for TrackedOp<T> {
    fn from(op: TensorOp<T>) -> Self {
        TrackedOp(Some(op))
    }
}

impl<T> From<TrackedOp<T>> for Option<TensorOp<T>> {
    fn from(op: TrackedOp<T>) -> Option<TensorOp<T>> {
        op.into_inner()
    }
}

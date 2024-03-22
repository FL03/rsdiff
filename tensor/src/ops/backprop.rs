/*
    Appellation: backprop <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::TensorOp;

pub struct BackpropOp<T>(Option<TensorOp<T>>);

impl<T> BackpropOp<T> {
    pub fn new(op: TensorOp<T>) -> Self {
        BackpropOp(Some(op))
    }

    pub fn none() -> Self {
        BackpropOp(None)
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

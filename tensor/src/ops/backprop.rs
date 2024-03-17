/*
    Appellation: backprop <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Op;

pub struct BackpropOp<T>(Option<Op<T>>);

impl<T> BackpropOp<T> {
    pub fn new(op: Op<T>) -> Self {
        BackpropOp(Some(op))
    }

    pub fn none() -> Self {
        BackpropOp(None)
    }

    pub fn op(&self) -> Option<&Op<T>> {
        self.0.as_ref()
    }

    pub fn op_mut(&mut self) -> Option<&mut Op<T>> {
        self.0.as_mut()
    }

    pub fn into_inner(self) -> Option<Op<T>> {
        self.0
    }
}

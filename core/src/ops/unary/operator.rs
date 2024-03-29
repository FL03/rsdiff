/*
   Appellation: operator <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Unary, UnaryOp};
// use std::marker::PhantomData;

pub struct UnaryOperator<A> {
    pub args: A,
    pub differentiable: bool,
    pub op: UnaryOp,
}

impl<A> UnaryOperator<A> {
    pub fn new(args: A, op: UnaryOp) -> Self {
        Self {
            args,
            differentiable: op.differentiable(),
            op,
        }
    }

    pub fn eval(self) -> A::Output
    where
        A: Unary,
    {
        self.args.unary(self.op)
    }
}

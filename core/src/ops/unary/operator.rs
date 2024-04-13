/*
   Appellation: operator <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::UnaryOp;

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
}

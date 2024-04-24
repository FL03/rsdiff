/*
   Appellation: operator <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::{Op, Params};

pub struct Operation<Args>
where
    Args: Params,
{
    pub args: Args,
    pub op: Op,
}

impl<Args> Operation<Args>
where
    Args: Params,
{
    pub fn new(args: Args, op: Op) -> Self {
        Self { args, op }
    }

    pub fn into_pattern(self) -> Args::Pattern {
        self.args.into_pattern()
    }

    pub fn args(&self) -> &Args {
        &self.args
    }

    pub fn op(&self) -> &Op {
        &self.op
    }
}

impl<A, B> Operation<crate::ops::binary::BinaryArgs<A, B>> {
    pub fn lhs(&self) -> &A {
        self.args.lhs()
    }

    pub fn rhs(&self) -> &B {
        self.args.rhs()
    }
}

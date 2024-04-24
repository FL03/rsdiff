/*
   Appellation: args <binary>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{BinaryOp, Operation};
use crate::ops::{IntoOp, Params};
use core::mem;

pub trait BinArgs: Params<Pattern = (Self::Lhs, Self::Rhs)> {
    type Lhs;
    type Rhs;

    fn apply(self, op: BinaryOp) -> Operation<Self>
    where
        Self: Sized,
    {
        Operation::new(self, op.into_op())
    }

    fn lhs(&self) -> &Self::Lhs;

    fn rhs(&self) -> &Self::Rhs;
}

pub struct BinaryArgs<A, B = A> {
    pub lhs: A,
    pub rhs: B,
}

impl<A, B> BinaryArgs<A, B> {
    pub fn new(lhs: A, rhs: B) -> Self {
        Self { lhs, rhs }
    }

    pub fn from_params<P>(params: P) -> Self
    where
        P: Params<Pattern = (A, B)>,
    {
        let (lhs, rhs) = params.into_pattern();
        Self::new(lhs, rhs)
    }

    pub fn into_args(self) -> (A, B) {
        (self.lhs, self.rhs)
    }

    pub fn flip(self) -> BinaryArgs<B, A> {
        BinaryArgs::new(self.rhs, self.lhs)
    }

    pub fn lhs(&self) -> &A {
        &self.lhs
    }

    pub fn rhs(&self) -> &B {
        &self.rhs
    }
}

impl<T> BinaryArgs<T, T> {
    pub fn swap(&mut self) {
        mem::swap(&mut self.lhs, &mut self.rhs);
    }
}

impl<A, B> BinArgs for BinaryArgs<A, B> {
    type Lhs = A;
    type Rhs = B;

    fn lhs(&self) -> &Self::Lhs {
        self.lhs()
    }

    fn rhs(&self) -> &Self::Rhs {
        self.rhs()
    }
}

impl<A, B> Params for BinaryArgs<A, B> {
    type Pattern = (A, B);

    fn into_pattern(self) -> Self::Pattern {
        (self.lhs, self.rhs)
    }
}

impl<A, B> From<BinaryArgs<A, B>> for (A, B) {
    fn from(args: BinaryArgs<A, B>) -> Self {
        args.into_pattern()
    }
}

impl<A, B> From<&BinaryArgs<A, B>> for (A, B)
where
    A: Clone,
    B: Clone,
{
    fn from(args: &BinaryArgs<A, B>) -> Self {
        (args.lhs.clone(), args.rhs.clone())
    }
}

impl<A, B> BinArgs for (A, B) {
    type Lhs = A;
    type Rhs = B;

    fn lhs(&self) -> &Self::Lhs {
        &self.0
    }

    fn rhs(&self) -> &Self::Rhs {
        &self.1
    }
}

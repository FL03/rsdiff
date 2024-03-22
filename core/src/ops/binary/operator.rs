/*
   Appellation: operator <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::BinaryOp;

pub struct BinaryArgs<A, B> {
    pub lhs: A,
    pub rhs: B,
}

impl<A, B> BinaryArgs<A, B> {
    pub fn new(lhs: A, rhs: B) -> Self {
        Self { lhs, rhs }
    }

    pub fn swap(self) -> BinaryArgs<B, A> {
        BinaryArgs::new(self.rhs, self.lhs)
    }

    pub fn lhs(&self) -> &A {
        &self.lhs
    }

    pub fn rhs(&self) -> &B {
        &self.rhs
    }
}

impl<A, B> From<BinaryArgs<A, B>> for (A, B) {
    fn from(args: BinaryArgs<A, B>) -> Self {
        (args.lhs, args.rhs)
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

impl<A, B> From<(A, B)> for BinaryArgs<A, B> {
    fn from((lhs, rhs): (A, B)) -> Self {
        Self::new(lhs, rhs)
    }
}

impl<A, B> From<&(A, B)> for BinaryArgs<A, B>
where
    A: Clone,
    B: Clone,
{
    fn from((lhs, rhs): &(A, B)) -> Self {
        Self::new(lhs.clone(), rhs.clone())
    }
}

pub struct BinaryOperator<A, B> {
    pub args: BinaryArgs<A, B>,
    pub communitative: bool,
    pub op: BinaryOp,
}

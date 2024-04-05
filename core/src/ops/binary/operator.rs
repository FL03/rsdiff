/*
   Appellation: operator <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::BinaryOp;
use core::marker::PhantomData;
use core::mem;

pub trait BinArgs {
    type Lhs;
    type Rhs;

    fn lhs(&self) -> &Self::Lhs;

    fn rhs(&self) -> &Self::Rhs;
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
pub struct BinaryArgs<A, B = A> {
    pub lhs: A,
    pub rhs: B,
}

impl<A, B> BinaryArgs<A, B> {
    pub fn new(lhs: A, rhs: B) -> Self {
        Self { lhs, rhs }
    }

    pub fn into_args(self) -> (A, B) {
        (self.lhs, self.rhs)
    }

    pub fn reverse(self) -> BinaryArgs<B, A> {
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

pub struct BinaryOperator<Args, C>
where
    Args: BinArgs,
{
    pub args: Args,
    pub communitative: bool,
    pub op: BinaryOp,
    pub output: PhantomData<C>,
}

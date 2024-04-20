/*
   Appellation: operator <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::BinaryOp;
use crate::ops::Params;

pub trait BinArgs: Params<Pattern = (Self::Lhs, Self::Rhs)> {
    type Lhs;
    type Rhs;

    fn apply(self, op: BinaryOp) -> BinaryOperator<Self>
    where
        Self: Sized,
    {
        BinaryOperator::new(self, op)
    }

    fn lhs(&self) -> &Self::Lhs;

    fn rhs(&self) -> &Self::Rhs;
}

pub struct BinaryArgs<A, B = A> {
    pub lhs: A,
    pub rhs: B,
}

pub struct BinaryOperator<Args>
where
    Args: BinArgs,
{
    pub args: Args,
    pub communitative: bool,
    pub op: BinaryOp,
}

impl<Args> BinaryOperator<Args>
where
    Args: BinArgs,
{
    pub fn new(args: Args, op: BinaryOp) -> Self {
        Self {
            args,
            communitative: op.is_commutative(),
            op,
        }
    }

    pub fn lhs(&self) -> &Args::Lhs {
        self.args.lhs()
    }

    pub fn rhs(&self) -> &Args::Rhs {
        self.args.rhs()
    }

    pub fn args(&self) -> &Args {
        &self.args
    }
}

mod impl_args {
    use super::{BinArgs, BinaryArgs};
    use crate::ops::Params;
    use core::mem;

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
}

/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Operations
//!
//!
pub use self::{arithmetic::*, kinds::*};

pub(crate) mod arithmetic;
pub(crate) mod kinds;

use std::marker::Tuple;

pub trait Differentiable<Args> {
    type Output;
    type Gradient;

    fn eval(&self, args: Args) -> Self::Output;

    fn grad(&self, args: Self::Output) -> Option<Self::Gradient>;
}

pub trait Evaluate<T> {
    type Output;

    fn eval(&self, args: T) -> Self::Output;
}

impl<A, B, S> Evaluate<A> for S
where
    S: Fn(A) -> B,
{
    type Output = B;

    fn eval(&self, args: A) -> Self::Output {
        self(args)
    }
}

pub trait Gradient<T> {
    type Gradient;

    fn grad(&self, args: T) -> Self::Gradient;
}



pub trait Operand<Args> where Args: Tuple {
    type Output;

    fn eval(&mut self, args: Args) -> Self::Output;
}

// impl<A, B, S> Operand<A> for S
// where
//     A: Tuple,
//     S: Fn(A) -> B,
// {
//     type Output = B;

//     fn compute(&self, args: A) -> Self::Output {
//         self(args)
//     }
// }

pub trait BinaryOperation<T> {
    type Output;

    fn eval(&self, lhs: T, rhs: T) -> Self::Output;
}

pub trait UnaryOperation<T> {
    type Output;

    fn eval(&self, arg: T) -> Self::Output;
}

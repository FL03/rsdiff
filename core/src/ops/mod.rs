/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Operations
//!
//!
pub use self::kinds::*;

pub(crate) mod kinds;

use std::marker::Tuple;

pub trait Operation<Args> {
    type Output;

    fn eval(&self, args: Args) -> Self::Output;
}

impl<A, B, O> Operation<A> for O
where
    O: Fn(A) -> B,
{
    type Output = B;

    fn eval(&self, args: A) -> Self::Output {
        self(args)
    }
}

pub trait Operand {}

pub trait BinaryOperation<T> {
    type Output;

    fn eval(&self, lhs: T, rhs: T) -> Self::Output;
}

pub trait UnaryOperation<T> {
    type Output;

    fn eval(&self, arg: T) -> Self::Output;
}

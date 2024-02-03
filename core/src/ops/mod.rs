/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Operations
//!
//!
pub use self::kinds::*;

pub(crate) mod kinds;



pub trait Differentiable<Args>: Gradient<Args> + Evaluate<Args> {}

impl<S, Args> Differentiable<Args> for S where S: Gradient<Args> + Evaluate<Args> {}

pub trait Evaluate<T> {
    type Output;

    fn eval(&self, args: T) -> Self::Output;
}

pub trait Gradient<T> {
    type Gradient;

    fn grad(&self, args: T) -> Self::Gradient;
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

pub trait Operand {}

pub trait BinaryOperation<T> {
    type Output;

    fn eval(&self, lhs: T, rhs: T) -> Self::Output;
}

pub trait UnaryOperation<T> {
    type Output;

    fn eval(&self, arg: T) -> Self::Output;
}

/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Operations
//!
//!
pub use self::{arithmetic::*, gradient::*, kinds::*, operator::*};

pub(crate) mod arithmetic;
pub(crate) mod gradient;
pub(crate) mod kinds;
pub(crate) mod operator;

pub trait Evaluate {
    type Output;

    fn eval(self) -> Self::Output;
}

impl Evaluate for f64 {
    type Output = f64;

    fn eval(self) -> Self::Output {
        self
    }
}

pub trait BinaryOperation<A, B> {
    type Output;

    fn eval(&self, lhs: A, rhs: B) -> Self::Output;
}

pub trait UnaryOperation {
    type Output;

    fn eval(self) -> Self::Output;
}

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

use crate::prelude::Result;

pub trait Expressive {
    type Graph;

    fn expand(&self) -> Self::Graph;
}

pub trait Backward {
    type Store;

    fn backward(&self) -> Result<Self::Store>;
}

pub trait Compute<T> {
    type Output;

    fn compute(&self, args: T) -> Self::Output;
}

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

pub trait BinaryOperation<T> {
    type Output;

    fn eval(&self, lhs: T, rhs: T) -> Self::Output;
}

pub trait UnaryOperation {
    type Output;

    fn eval(self) -> Self::Output;
}

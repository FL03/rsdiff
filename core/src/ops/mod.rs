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

use crate::prelude::Result;
use std::marker::Tuple;

pub trait Backward {
    type Store;

    fn backward(&self) -> Result<Self::Store>;
}

pub trait Compute<T> {
    type Output;

    fn compute(&self, args: T) -> Self::Output;
}

pub trait Differentiable<T> {
    type Derivative;

    fn eval(&self, at: T) -> T;
    fn derivative(&self, at: T) -> Self::Derivative;
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

pub trait Gradient<T>
where
    T: Gradient<T>,
{
    type Gradient;

    fn grad(&self, args: T) -> Self::Gradient;
}

pub trait Operand<Args>
where
    Args: Tuple,
{
    type Output;

    fn name(&self) -> &str;

    fn eval(&self, args: Args) -> Self::Output;

    fn grad(&self, args: Self::Output) -> Option<Self::Output>;
}

pub trait BinaryOperation<T> {
    type Output;

    fn eval(&self, lhs: T, rhs: T) -> Self::Output;
}

pub trait UnaryOperation {
    type Output;

    fn eval(self) -> Self::Output;
}

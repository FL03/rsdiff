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

pub mod binary;
pub mod unary;

pub trait BinaryOperation<A, B> {
    type Output;

    fn eval(&self, lhs: A, rhs: B) -> Self::Output;
}

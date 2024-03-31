/*
   Appellation: unary <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Unary Operations
//!
//!
pub use self::{kinds::*, operator::*, specs::*};

pub(crate) mod kinds;
pub(crate) mod operator;
pub(crate) mod specs;

pub trait Unary {
    type Output;

    fn name(&self) -> &str;

    fn unary(self, expr: UnaryOp) -> Self::Output;
}

#[cfg(test)]
mod tests {}

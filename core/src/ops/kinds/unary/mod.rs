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

pub trait ApplyUnary<T> {
    type Output;

    fn apply(&self, x: T) -> Self::Output;

    fn apply_once(self, x: T) -> Self::Output;
}

/*
   Appellation: unary <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Unary Operations
//!
//!
pub use self::kinds::*;

pub(crate) mod kinds;

pub trait UnaryOperation {
    type Output;

    fn eval(self) -> Self::Output;
}

#[cfg(test)]
mod tests {}

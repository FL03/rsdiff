/*
   Appellation: binary <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{kinds::*, operator::*};

pub(crate) mod kinds;
pub(crate) mod operator;

pub trait BinOp<A, B> {
    type Output;

    fn apply(lhs: A, rhs: B) -> Self::Output;
}

#[cfg(test)]
mod tests {}

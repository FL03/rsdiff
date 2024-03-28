/*
    Appellation: iter <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Iter
//!
//!
pub use self::iterator::*;

pub(crate) mod iterator;

pub trait IterTensor {
    type Item;
}

#[cfg(test)]
mod tests {}

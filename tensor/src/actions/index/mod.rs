/*
    Appellation: index <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Index
//!
//!
pub use self::slice::*;

pub(crate) mod slice;

/// A type alias for an unsigned integer used to index into a tensor.
pub type Ix = usize;
/// A type alias for a signed integer used to index into a tensor.
pub type Ixs = isize;

pub trait TensorIndex {}

#[cfg(test)]
mod tests {}

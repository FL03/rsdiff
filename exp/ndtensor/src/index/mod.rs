/*
    Appellation: index <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Index
//!
//!
pub use self::slice::*;

pub(crate) mod slice;

pub type Ix = usize;

pub type Ixs = isize;

#[cfg(test)]
mod tests {}

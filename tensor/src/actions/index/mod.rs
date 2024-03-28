/*
    Appellation: index <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Index
//!
//!
pub use self::slice::*;

pub(crate) mod slice;

pub trait TensorIdx {}

#[cfg(test)]
mod tests {}

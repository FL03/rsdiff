/*
    Appellation: ids <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Ids
//!
//!
pub use self::{atomic::*, id::*};

pub(crate) mod atomic;
pub(crate) mod id;

pub trait Identifier {}

#[cfg(test)]
mod tests {}

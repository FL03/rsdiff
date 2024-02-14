/*
    Appellation: ids <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Ids
//!
//!
pub use self::{atomic::*, gradient::*, id::*};

pub(crate) mod atomic;
pub(crate) mod gradient;
pub(crate) mod id;

#[cfg(test)]
mod tests {}

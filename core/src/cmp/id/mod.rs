/*
    Appellation: ids <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Ids
//!
//!
pub use self::{gradient::*, id::*};

pub(crate) mod gradient;
pub(crate) mod id;

#[cfg(test)]
mod tests {}

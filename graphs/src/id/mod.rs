/*
    Appellation: ids <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Ids
//!
//!
pub use self::{entry::*, id::*};

pub(crate) mod entry;
pub(crate) mod id;

pub trait Identifier {}

pub trait Index {
    fn next(&self) -> Self;
}

#[cfg(test)]
mod tests {}

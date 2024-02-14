/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Errors
//!
//!
pub use self::{error::*, kinds::*};

pub(crate) mod error;
pub(crate) mod kinds;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {}

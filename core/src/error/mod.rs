/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Errors
//!
//!
pub use self::{err::*, kinds::*};

pub(crate) mod err;
pub(crate) mod kinds;

pub type Result<T = ()> = core::result::Result<T, Error>;

#[cfg(test)]
mod tests {}

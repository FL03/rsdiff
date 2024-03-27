/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Operations
//!
//!
pub use self::kinds::*;

pub(crate) mod kinds;

pub mod binary;
pub mod unary;

pub trait Operation {
    type Output;

    fn kind(&self) -> String;
}

pub(crate) mod prelude {
    pub use super::binary::*;
    pub use super::kinds::Op;
    pub use super::unary::*;
    pub use super::Operation;
}

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

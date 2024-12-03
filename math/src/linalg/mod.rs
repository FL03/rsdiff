/*
    Appellation: linalg <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Linear Algebra
//!
//! This module implements fundamental linear algebra concepts and operations.
//!
pub use self::fields::*;

pub(crate) mod fields;

pub mod vs;

#[cfg(test)]
mod tests {}

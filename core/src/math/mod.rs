/*
    Appellation: math <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Mathematics
//!
//! This module implements fundamental mathematical concepts and operations.
//! Each sub-module is dedicated to a specific branch of mathematics.
pub use self::props::*;

pub(crate) mod props;

pub mod linalg;

pub trait Group {}

#[cfg(test)]
mod tests {}

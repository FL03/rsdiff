/*
    Appellation: math <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Mathematics
//!
//! This module implements fundamental mathematical concepts and operations.
//! Each sub-module is dedicated to a specific branch of mathematics.
pub use self::props::*;

#[doc(hidden)]
pub mod cluster;
pub mod linalg;
pub mod num;
pub mod props;
#[doc(hidden)]
pub mod signal;
#[doc(hidden)]
pub mod stats;

pub trait Group {}

#[cfg(test)]
mod tests {}

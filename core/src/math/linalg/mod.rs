/*
    Appellation: linalg <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Linear Algebra
//!
//! This module implements fundamental linear algebra concepts and operations.
//!
pub mod fields;

pub trait VectorSpace {}

pub trait Subspace: VectorSpace {}

#[cfg(test)]
mod tests {}

/*
    Appellation: actions <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Actions
//!
//! This module contains the implementations of the various actions that can be performed on tensors.
//! The actions include:
//!     - Composition
//!     - Differentiation
//!     - Indexing
//!     - Iteration

pub mod arange;
pub mod grad;
pub mod index;
pub mod iter;

pub(crate) mod prelude {
    pub use super::arange::*;
}

#[cfg(test)]
mod tests {}

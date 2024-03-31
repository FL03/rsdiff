/*
    Appellation: actions <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Actions
//!
//! This module describes the actions that may be taken on or by a tensor.
//!
//! The actions include:<br>
//! * Automatic Differentiation
//! * Creation Routines (``)
//! * Indexing
//! * Iteration

pub mod create;
pub mod grad;
pub mod index;
pub mod iter;

pub(crate) mod prelude {
    pub use super::create::*;
    pub use super::grad::*;
    pub use super::index::*;
    pub use super::iter::*;
}

#[cfg(test)]
mod tests {}

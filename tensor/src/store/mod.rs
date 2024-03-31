/*
    Appellation: store <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Store
//!
//! This module provides the storage and layout for the tensor data structure.
pub use self::layout::Layout;

pub(crate) mod layout;

#[cfg(test)]
mod tests {}

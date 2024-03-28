/*
    Appellation: store <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Store
//!
//! This module provides the storage and layout for the tensor data structure.
pub use self::{layout::*, storage::*};

pub(crate) mod layout;
pub(crate) mod storage;

pub trait TensorStore {
    type Elem;
}

#[cfg(test)]
mod tests {}

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

pub enum TensorData<T> {
    Scalar(T),
    Tensor(Vec<TensorData<T>>),
}

pub enum TensorBackend {
    Scalar,
    Tensor,
}

#[cfg(test)]
mod tests {}

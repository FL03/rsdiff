/*
    Appellation: index <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Index
//!
//!
pub use self::slice::*;

pub(crate) mod slice;

use crate::tensor::TensorBase;

pub type Ix = usize;

pub type Ixs = isize;

pub enum IndexItem<T> {
    Scalar(T),
    Tensor(TensorBase<T>),
}

pub trait TensorIndex {
    
}

#[cfg(test)]
mod tests {}

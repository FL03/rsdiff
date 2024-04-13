/*
    Appellation: ndtensor <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # ndtensor
//!
//!
#![crate_name = "ndtensor"]

extern crate acme_core as acme;

pub use self::tensor::*;

pub(crate) mod tensor;

pub mod ops;

pub type NdContainer<S> = ndarray::ArrayBase<S, ndarray::IxDyn>;

pub mod prelude {
    pub use crate::tensor::Tensor;
}

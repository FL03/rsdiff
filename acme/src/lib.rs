/*
    Appellation: acme <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme
//!
//! Acme is an autodifferentiaion library for Rust. It is designed to be a
//! flexible and powerful tool for building machine learning models and
//! other differentiable programs.

#[cfg(feature = "core")]
pub use acme_core as core;
#[cfg(feature = "derive")]
pub use acme_derive::*;
#[cfg(feature = "macros")]
pub use acme_macros::*;
#[cfg(feature = "tensor")]
pub use acme_tensor as tensor;

pub mod prelude {
    #[cfg(feature = "core")]
    pub use crate::core::prelude::*;
    #[cfg(feature = "derive")]
    pub use acme_derive::*;
    #[cfg(feature = "macros")]
    pub use acme_macros::*;
    #[cfg(feature = "tensor")]
    pub use crate::tensor::prelude::*;
}

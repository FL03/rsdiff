/*
    Appellation: acme <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme
//!
//! Acme is an autodifferentiaion library for Rust. It is designed to be a
//! flexible and powerful tool for building machine learning models and
//! other differentiable programs.
#![crate_name = "acme"]

#[doc(inline)]
pub use acme_core::*;
#[cfg(feature = "derive")]
pub use acme_derive::*;
#[cfg(feature = "graph")]
#[doc(inline)]
pub use acme_graphs as graph;
#[cfg(feature = "macros")]
pub use acme_macros::*;
#[cfg(feature = "tensor")]
#[doc(inline)]
pub use acme_tensor as tensor;

pub mod prelude {
    #[doc(inline)]
    pub use acme_core::prelude::*;
    #[cfg(feature = "derive")]
    pub use acme_derive::*;
    #[cfg(feature = "graph")]
    #[doc(inline)]
    pub use acme_graphs::prelude::*;
    #[cfg(feature = "macros")]
    pub use acme_macros::*;
    #[cfg(feature = "tensor")]
    #[doc(inline)]
    pub use acme_tensor::prelude::*;
}

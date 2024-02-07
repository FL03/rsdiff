/*
    Appellation: acme-tensor <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme-tensor
//!
//!
#![feature(fn_traits, tuple_trait, unboxed_closures)]
pub use self::tensor::*;

pub(crate) mod tensor;

pub mod data;
pub mod shape;
pub mod store;

pub(crate) use acme_core as core;

pub mod prelude {
    pub use crate::data::*;
    pub use crate::shape::*;
    pub use crate::store::*;
    pub use crate::tensor::*;
}

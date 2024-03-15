/*
    Appellation: acme-tensor <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme-tensor
//!
//!
#![feature(array_chunks)]
#[cfg(not(feature = "std"))]
extern crate alloc;

extern crate acme_core as acme;

pub use self::tensor::*;

pub(crate) mod tensor;

pub mod data;
pub mod ops;
pub mod shape;
pub mod specs;
pub mod store;

pub mod prelude {
    pub use crate::tensor::TensorBase;

    pub use crate::data::*;
    pub use crate::ops::*;
    pub use crate::shape::*;
    pub use crate::specs::prelude::*;
    pub use crate::store::*;
}

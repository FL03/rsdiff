/*
    Appellation: acme-tensor <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme-tensor
//!
//!
#![feature(fn_traits, tuple_trait, unboxed_closures)]
pub use self::{specs::*, tensor::*};

pub(crate) mod specs;
pub(crate) mod tensor;

pub mod data;
pub mod ops;
pub mod shape;
pub mod store;

pub(crate) use acme_core as core;

pub mod prelude {
    pub use crate::specs::*;

    pub use crate::data::*;
    pub use crate::ops::*;
    pub use crate::shape::*;
    pub use crate::store::*;
    pub use crate::tensor::*;
}

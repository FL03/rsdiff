/*
    Appellation: acme-tensor <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme-tensor
//!
//!
#![feature(array_chunks)]

extern crate acme_core as acme;
pub use self::{specs::*, tensor::*};

pub(crate) mod specs;
pub(crate) mod tensor;

pub mod data;
pub mod ops;
pub mod shape;
pub mod store;

pub mod prelude {
    pub use crate::specs::*;

    pub use crate::data::*;
    pub use crate::ops::*;
    pub use crate::shape::*;
    pub use crate::store::*;
    pub use crate::tensor::*;
}

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

mod impls {
    mod arith;
    mod grad;
    mod linalg;
}

pub type Tensor<T = f64> = tensor::TensorBase<T>;

pub mod prelude {
    #[doc(inline)]
    pub use crate::data::*;
    #[doc(inline)]
    pub use crate::ops::*;
    #[doc(inline)]
    pub use crate::shape::*;
    #[doc(inline)]
    pub use crate::specs::prelude::*;
    #[doc(inline)]
    pub use crate::store::*;
    #[doc(inline)]
    pub use crate::Tensor;
}

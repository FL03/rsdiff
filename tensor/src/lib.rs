/*
    Appellation: acme-tensor <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Tensor
//!
//! This library implements a tensor data structure with support for automatic differentiation.
//!
#[cfg(not(feature = "std"))]
extern crate alloc;

extern crate acme_core as acme;

#[doc(inline)]
pub use self::{tensor::*, utils::*};

#[macro_use]
pub(crate) mod seal;
pub(crate) mod tensor;
#[macro_use]
pub(crate) mod utils;

pub mod actions;
pub mod backend;
pub mod data;
pub mod error;
pub mod linalg;
pub mod ops;
pub mod shape;
pub mod specs;
pub mod stats;
pub mod store;
pub mod types;

mod impls {
    mod ops {
        mod binary;
        mod unary;
    }
    mod create;
    mod grad;
    mod linalg;
    mod num;
    mod reshape;
}

pub type Tensor<T = f64> = tensor::TensorBase<T>;

pub mod prelude {
    #[doc(inline)]
    pub use crate::actions::prelude::*;
    #[doc(inline)]
    pub use crate::data::prelude::*;
    #[doc(inline)]
    pub use crate::error::*;
    #[doc(inline)]
    pub use crate::linalg::prelude::*;
    #[doc(inline)]
    pub use crate::ops::*;
    #[doc(inline)]
    pub use crate::shape::prelude::*;
    #[doc(inline)]
    pub use crate::specs::prelude::*;
    #[doc(inline)]
    pub use crate::store::*;
    #[doc(inline)]
    pub use crate::types::prelude::*;
    pub use crate::utils::*;
    #[doc(inline)]
    pub use crate::Tensor;
}

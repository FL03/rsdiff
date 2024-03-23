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
pub use self::{error::*, tensor::*};

pub(crate) mod error;
pub(crate) mod tensor;

pub mod actions;
pub mod data;
pub mod linalg;
pub mod ops;
pub mod shape;
pub mod specs;
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
    pub use crate::data::*;
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
    #[doc(inline)]
    pub use crate::Tensor;
}

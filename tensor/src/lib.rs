/*
    Appellation: acme-tensor <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme-tensor
//!
//!
#[cfg(not(feature = "std"))]
extern crate alloc;

extern crate acme_core as acme;

#[doc(inline)]
pub use self::tensor::*;

pub(crate) mod tensor;

pub mod actions;
pub mod data;
pub mod errors;
pub mod linalg;
pub mod ops;
pub mod shape;
pub mod specs;
pub mod store;

mod impls {
    mod ops {
        mod binary;
        mod unary;
    }
    mod grad;
    mod linalg;
    mod num;
}

pub type Tensor<T = f64> = tensor::TensorBase<T>;

pub mod prelude {
    #[doc(inline)]
    pub use crate::data::*;
    #[doc(inline)]
    pub use crate::errors::*;
    #[doc(inline)]
    pub use crate::ops::*;
    #[doc(inline)]
    pub use crate::shape::prelude::*;
    #[doc(inline)]
    pub use crate::specs::prelude::*;
    #[doc(inline)]
    pub use crate::store::*;
    #[doc(inline)]
    pub use crate::Tensor;
}

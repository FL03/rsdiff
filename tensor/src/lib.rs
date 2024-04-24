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
pub use self::{actions::*, tensor::*, utils::*};

#[allow(unused)]
#[macro_use]
pub(crate) mod seal;
pub(crate) mod tensor;
#[macro_use]
pub(crate) mod utils;

#[doc(hidden)]
pub mod backend;
pub mod error;
#[doc(hidden)]
pub mod fmt;
#[cfg(feature = "io")]
pub mod io;
pub mod linalg;
pub mod ops;
pub mod shape;
pub mod specs;
pub mod stats;
pub mod types;

pub(crate) mod actions {

    pub mod create;
    pub mod grad;
    pub mod index;
    pub mod iter;
}

mod impls {
    mod ops {
        mod binary;
        mod unary;
    }
    mod create;
    mod grad;
    mod iter;
    mod linalg;
    mod num;
    mod reshape;
}

pub type Tensor<T = f64> = tensor::TensorBase<T>;

pub mod prelude {
    pub use crate::actions::{create::*, grad::*, index::*, iter::*};
    pub use crate::error::*;
    pub use crate::linalg::prelude::*;
    pub use crate::ops::*;
    pub use crate::shape::prelude::*;
    pub use crate::specs::prelude::*;
    pub use crate::stats::prelude::*;
    pub use crate::types::prelude::*;
    pub use crate::utils::*;
    pub use crate::Tensor;
    pub(crate) use acme::prelude::Scalar;
}

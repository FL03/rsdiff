/*
    Appellation: ndtensor <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # ndtensor
//!
//!
#![crate_name = "ndtensor"]

extern crate acme_core as acme;

pub use self::{errors::*, specs::*, tensor::*, utils::*};

pub(crate) mod errors;
pub(crate) mod specs;
pub(crate) mod tensor;
pub(crate) mod utils;

pub mod ops;

pub type TensorId = acme::id::AtomicId;

pub type NdContainer<S> = ndarray::ArrayBase<S, ndarray::IxDyn>;

pub mod prelude {
    pub use crate::errors::{TensorError, TensorResult};
    pub use crate::specs::NdTensor;
    pub use crate::tensor::Tensor;
    pub use crate::utils::*;
    pub use crate::{NdContainer, TensorId};

    #[allow(unused_imports)]
    pub(crate) use ndarray::{
        array, s, ArrayBase, ArrayD, Data, DataOwned, Dimension, IxDyn, RawData, ShapeError,
    };
}

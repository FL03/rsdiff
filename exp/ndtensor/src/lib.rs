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

pub(crate) mod impls {
    pub(crate) mod ops;
}

use ndarray::{CowRepr, IxDyn, OwnedArcRepr, OwnedRepr, ViewRepr};

pub type ArcTensor<S, D = IxDyn> = TensorBase<OwnedArcRepr<S>, D>;

pub type CowTensor<'a, S, D = IxDyn> = TensorBase<CowRepr<'a, S>, D>;

pub type Tensor<S, D = IxDyn> = TensorBase<OwnedRepr<S>, D>;

pub type TensorView<'a, S, D = IxDyn> = TensorBase<ViewRepr<&'a S>, D>;

pub type TensorViewMut<'a, S, D = IxDyn> = TensorBase<ViewRepr<&'a mut S>, D>;

pub type TensorId = acme::id::AtomicId;

pub type NdContainer<S> = ndarray::ArrayBase<S, ndarray::IxDyn>;

pub mod prelude {
    pub use crate::errors::{TensorError, TensorResult};
    pub use crate::ops::{TensorExpr, TensorOp};
    pub use crate::specs::NdTensor;
    pub use crate::tensor::TensorBase;
    pub use crate::utils::*;
    pub use crate::{
        ArcTensor, CowTensor, NdContainer, Tensor, TensorId, TensorView, TensorViewMut,
    };

    #[allow(unused_imports)]
    pub(crate) use ndarray::{
        array, s, ArrayBase, ArrayD, Data, DataOwned, Dimension, IxDyn, RawData, ShapeError,
    };
}

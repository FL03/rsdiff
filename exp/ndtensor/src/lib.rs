/*
    Appellation: ndtensor <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # ndtensor
//!
//!
#![crate_name = "ndtensor"]

extern crate acme_core as acme;

pub use self::{context::Context, errors::*, specs::*, tensor::*, types::*, utils::*};

pub(crate) mod context;
pub(crate) mod errors;
#[macro_use]
pub(crate) mod macros;
pub(crate) mod specs;
pub(crate) mod tensor;
pub(crate) mod utils;

pub mod ops;

#[allow(unused_imports)]
pub(crate) mod nd {
    pub(crate) use ndarray::*;
}

pub(crate) mod impls {
    pub(crate) mod create;
    pub(crate) mod grad;
    pub(crate) mod ops;
}

pub(crate) mod types {
    pub use self::kinds::*;
    pub(crate) mod kinds;
}

use ndarray::{CowRepr, IxDyn, OwnedArcRepr, OwnedRepr, ViewRepr};

pub type ArcTensor<A, D = IxDyn> = TensorBase<OwnedArcRepr<A>, D>;

pub type CowTensor<'a, A, D = IxDyn> = TensorBase<CowRepr<'a, A>, D>;

pub type RawTensorView<A, D = IxDyn> = TensorBase<ndarray::RawViewRepr<*const A>, D>;

pub type Tensor<S, D = IxDyn> = TensorBase<OwnedRepr<S>, D>;

pub type TensorView<'a, S, D = IxDyn> = TensorBase<ViewRepr<&'a S>, D>;

pub type TensorViewMut<'a, S, D = IxDyn> = TensorBase<ViewRepr<&'a mut S>, D>;

pub type TensorId = acme::id::AtomicId;

pub type NdContainer<S> = ndarray::ArrayBase<S, ndarray::IxDyn>;

pub mod prelude {
    pub use crate::errors::{TensorError, TensorResult};
    pub use crate::ops::{TensorExpr, TensorOp};
    pub use crate::specs::*;
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

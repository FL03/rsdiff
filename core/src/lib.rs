/*
    Appellation: acme-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme-core
//!
//!
#![allow(incomplete_features)]
#![feature(adt_const_params, fn_traits, tuple_trait, unboxed_closures)]

pub use self::primitives::*;

pub(crate) mod primitives;
pub(crate) mod specs;
pub(crate) mod utils;

pub(crate) mod exp;

pub mod cmp;
pub mod errors;
pub mod graphs;
pub mod hkt;
pub mod ops;
pub mod stores;

pub mod prelude {
    pub use crate::primitives::*;
    // pub use crate::specs::*;
    // pub use crate::utils::*;

    pub use crate::cmp::*;
    pub use crate::errors::*;
    pub use crate::graphs::*;
    pub use crate::ops::*;
    pub use crate::stores::*;
}

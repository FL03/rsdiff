/*
    Appellation: rsdiff <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rsdiff
//!
//! rsdiff is an autodifferentiaion library for Rust. It is designed to be a
//! flexible and powerful tool for building machine learning models and
//! other differentiable programs.
#![crate_name = "rsdiff"]

#[doc(inline)]
pub use rsdiff_core::*;
#[allow(unused_imports)]
#[cfg(feature = "derive")]
#[doc(inline)]
pub use rsdiff_derive::*;
#[cfg(feature = "graph")]
#[doc(inline)]
pub use rsdiff_graphs as graph;
#[cfg(feature = "macros")]
#[doc(inline)]
pub use rsdiff_macros::*;
#[cfg(feature = "math")]
#[doc(inline)]
pub use rsdiff_math as math;

pub mod prelude {
    pub use rsdiff_core::prelude::*;
    #[allow(unused_imports)]
    #[cfg(feature = "derive")]
    pub use rsdiff_derive::*;
    #[cfg(feature = "graph")]
    pub use rsdiff_graphs::prelude::*;
    #[cfg(feature = "macros")]
    pub use rsdiff_macros::*;
    #[cfg(feature = "math")]
    pub use rsdiff_math::prelude::*;
}

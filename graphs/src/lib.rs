/*
    Appellation: acme-graphs <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # acme-graphs
//!
//!

extern crate acme_core as acme;

#[doc(inline)]
pub use self::graph::*;

pub(crate) mod graph;

pub mod dcg;
pub mod errors;
pub mod grad;
pub mod ops;
pub mod scg;

pub use petgraph::graph::{EdgeIndex, GraphIndex, NodeIndex};

pub mod prelude {
    #[doc(inline)]
    pub use crate::dcg::Dcg;
    #[doc(inline)]
    pub use crate::errors::*;
    #[doc(inline)]
    pub use crate::grad::prelude::*;
    #[doc(inline)]
    pub use crate::graph::*;
    #[doc(inline)]
    pub use crate::scg::Scg;
}

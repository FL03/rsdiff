/*
    Appellation: rsdiff-graphs <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # rsdiff-graphs
//!
//!

extern crate rsdiff_core as rsdiff;

#[doc(inline)]
pub use self::graph::*;

pub(crate) mod graph;
#[macro_use]
pub(crate) mod macros;

#[doc(hidden)]
pub mod cg;
pub mod dcg;
pub mod error;
pub mod grad;
pub mod scg;

pub use petgraph::graph::{EdgeIndex, GraphIndex, NodeIndex};
pub use petgraph::stable_graph::DefaultIx;

pub(crate) type Id = rsdiff::id::IndexId<crate::NodeIndex>;

#[doc(hidden)]
pub mod prelude {
    #[doc(inline)]
    pub use crate::dcg::Dcg;
    #[doc(inline)]
    pub use crate::error::{GraphError, GraphResult};
    #[doc(inline)]
    pub use crate::grad::prelude::*;
    #[doc(inline)]
    pub use crate::graph::*;
    #[doc(inline)]
    pub use crate::scg::Scg;
}

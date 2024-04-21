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
pub mod error;
pub mod grad;
pub mod id;
pub mod ops;
pub mod scg;

pub use petgraph::graph::{EdgeIndex, GraphIndex, NodeIndex};

pub type Gid = acme::id::IndexId<crate::NodeIndex>;


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

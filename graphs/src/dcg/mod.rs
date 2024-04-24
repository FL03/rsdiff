/*
    Appellation: dcg <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Dynamic Compute Graph
//!
//! A computational graph forms the backbone of automatic differentiation. Computational graphs are directed acyclic graphs (DAGs)
//! that represent any computation as a series of nodes and edges.
pub use self::{edge::Edge, graph::Dcg, node::Node};

pub(crate) mod graph;

pub mod edge;
pub mod node;

pub(crate) type DynamicGraph<T> = petgraph::graph::DiGraph<node::Node<T>, edge::Edge>;

pub trait GraphData {
    type Value: ?Sized;
}

impl<S> GraphData for S
where
    S: acme::prelude::Scalar<Real = S>,
{
    type Value = S;
}

#[cfg(test)]
mod tests {}

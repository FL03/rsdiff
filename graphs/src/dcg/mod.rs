/*
    Appellation: dcg <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Dynamic Compute Graph
//!
//! A computational graph forms the backbone of automatic differentiation. Computational graphs are directed acyclic graphs (DAGs)
//! that represent any computation as a series of nodes and edges.
pub use self::graph::Dcg;

pub(crate) mod graph;

pub mod edge;
pub mod node;

pub(crate) type DynamicGraph<T> = petgraph::graph::DiGraph<node::Node<T>, edge::Edge>;

#[cfg(test)]
mod tests {}

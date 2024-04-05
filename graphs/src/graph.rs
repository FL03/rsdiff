/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Graph
//!
//! A computational graph forms the backbone of automatic differentiation. Computational graphs are directed acyclic graphs (DAGs)
//! that represent any computation as a series of nodes and edges.
//!
//! In a dynamic computational graph (DCG), the graph considers the nodes to be tensors and the edges to be operations.
//!

pub trait GraphEntry {
    type Idx;
    type Weight;
}

pub trait ComputeGraph {
    type Edge: GraphEntry;
    type Node: GraphEntry;

    fn add_node(
        &mut self,
        node: <Self::Node as GraphEntry>::Weight,
    ) -> <Self::Node as GraphEntry>::Idx;

    fn add_edge(
        &mut self,
        source: <Self::Node as GraphEntry>::Idx,
        target: <Self::Node as GraphEntry>::Idx,
        weight: <Self::Edge as GraphEntry>::Weight,
    ) -> <Self::Edge as GraphEntry>::Idx;

    fn clear(&mut self);
}

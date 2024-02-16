/*
    Appellation: graphs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Graphs
//!
//! A computational graph forms the backbone of automatic differentiation. Computational graphs are directed acyclic graphs (DAGs)
//! that represent any computation as a series of nodes and edges.
//!
//! In a dynamic computational graph (DCG), the graph considers the nodes to be tensors and the edges to be operations.
//!

pub mod scg;

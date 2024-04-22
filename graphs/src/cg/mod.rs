/*
    Appellation: cg <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{edge::Edge, graph::*, node::Node};

pub(crate) mod graph;

pub mod edge;
pub mod node;

pub(crate) type CGraph<T> = petgraph::graph::DiGraph<Node<T>, Edge>;

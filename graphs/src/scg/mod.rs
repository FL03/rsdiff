/*
    Appellation: scg <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Static Computational Graph
//!
//!
pub use self::{edge::*, graph::*, node::*};

pub(crate) mod edge;
pub(crate) mod graph;
pub(crate) mod node;

#[cfg(test)]
mod tests {}

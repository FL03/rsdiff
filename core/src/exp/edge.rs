/*
    Appellation: value <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Edges
//!
//! In computational graphs, edges
use daggy::NodeIndex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Edge<T> {
    pub deps: Vec<NodeIndex>,
    pub value: T,
}

impl<T> Edge<T> {
    pub fn new(deps: Vec<NodeIndex>, value: T) -> Self {
        Self { deps, value }
    }
}

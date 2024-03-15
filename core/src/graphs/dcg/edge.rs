/*
    Appellation: edge <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use petgraph::graph::NodeIndex;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Edge {
    source: NodeIndex,
}

impl Edge {
    pub fn new(source: NodeIndex) -> Self {
        Self { source }
    }

    pub fn source(&self) -> NodeIndex {
        self.source
    }
}

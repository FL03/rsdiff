/*
    Appellation: edge <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use petgraph::graph::NodeIndex;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Edge<Idx = NodeIndex> {
    source: Idx,
}

impl<Idx> Edge<Idx> {
    pub fn new(source: Idx) -> Self {
        Self { source }
    }

    pub fn source(&self) -> &Idx {
        &self.source
    }

    pub fn into_source(self) -> Idx {
        self.source
    }
}

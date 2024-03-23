/*
    Appellation: edge <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::id::Id;
use crate::NodeIndex;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Edge<Idx = NodeIndex> {
    source: Id<Idx>,
}

impl<Idx> Edge<Idx> {
    pub fn new(source: Idx) -> Self {
        Self {
            source: Id::new(source),
        }
    }

    pub fn get_id(&self) -> usize {
        self.source.id()
    }

    pub fn get_index(&self) -> &Idx {
        self.source.index()
    }

    pub fn source(&self) -> &Id<Idx> {
        &self.source
    }

    pub fn into_source(self) -> Id<Idx> {
        self.source
    }
}

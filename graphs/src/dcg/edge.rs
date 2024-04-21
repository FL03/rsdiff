/*
    Appellation: edge <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::NodeIndex;
use acme::id::IndexId;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Edge<Idx = NodeIndex> {
    source: IndexId<Idx>,
}

impl<Idx> Edge<Idx> {
    pub fn new(source: Idx) -> Self {
        Self {
            source: IndexId::from_index(source),
        }
    }

    pub fn get_id(&self) -> usize {
        self.source.id()
    }

    pub fn get_index(&self) -> &Idx {
        self.source.index()
    }

    pub fn source(&self) -> &IndexId<Idx> {
        &self.source
    }

    pub fn into_source(self) -> IndexId<Idx> {
        self.source
    }
}

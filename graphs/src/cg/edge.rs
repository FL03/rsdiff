/*
    Appellation: edge <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::NodeIndex;
use acme::id::IndexId;
use acme::ops::Op;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Edge<Idx = NodeIndex> {
    args: Vec<IndexId<Idx>>,
    op: Op,
}

impl<Idx> Edge<Idx> {
    pub fn new(args: impl IntoIterator<Item = Idx>, op: impl Into<Op>) -> Self {
        Self {
            args: Vec::from_iter(args.into_iter().map(IndexId::from_index)),
            op: op.into(),
        }
    }

    pub fn args(&self) -> &[IndexId<Idx>] {
        &self.args
    }

    pub fn args_mut(&mut self) -> &mut [IndexId<Idx>] {
        &mut self.args
    }

    pub fn args_iter(&self) -> core::slice::Iter<'_, IndexId<Idx>> {
        self.args.iter()
    }

    pub fn op(&self) -> &Op {
        &self.op
    }
}

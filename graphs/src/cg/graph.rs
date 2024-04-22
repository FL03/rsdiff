/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{CGraph, Edge};
use crate::NodeIndex;
use acme::ops::BinaryOp;

pub struct Graph<T> {
    store: CGraph<T>,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Self {
            store: CGraph::new(),
        }
    }

    pub fn binary(&mut self, lhs: NodeIndex, rhs: NodeIndex, op: BinaryOp) {
        let _a = &self.store[lhs];
        let _b = &self.store[rhs];
        let edge = Edge::new([lhs, rhs], op.into());
        self.store.add_edge(lhs, rhs, edge);
    }
}

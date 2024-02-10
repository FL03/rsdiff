/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Node
//!
//! A computational graph relies on weighted nodes to represent constants, operations, and variables.
//! The edges connecting to any given node are considered to be inputs and help to determine the flow of information
use crate::ops::Ops;
use daggy::NodeIndex;

#[derive(Clone, Debug, Default)]
pub struct Node {
    inputs: Vec<NodeIndex>,
    op: Option<Ops>,
}

impl Node {
    pub fn new(inputs: impl IntoIterator<Item = NodeIndex>, op: Option<Ops>) -> Self {
        Self {
            inputs: Vec::from_iter(inputs),
            op,
        }
    }

    pub fn clear(&mut self) {
        self.inputs.clear();
        self.op = None;
    }

    pub fn inputs(&self) -> &[NodeIndex] {
        &self.inputs
    }

    pub fn inputs_mut(&mut self) -> &mut Vec<NodeIndex> {
        &mut self.inputs
    }

    pub fn operation(&self) -> Option<&Ops> {
        self.op.as_ref()
    }
}

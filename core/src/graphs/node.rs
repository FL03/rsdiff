/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Node
//!
//! A computational graph relies on weighted nodes to represent constants, operations, and variables.
//! The edges connecting to any given node are considered to be inputs and help to determine the flow of information
use crate::cmp::id::AtomicId;
use crate::ops::Ops;
use daggy::NodeIndex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Node {
    id: AtomicId,
    inputs: Vec<NodeIndex>,
    name: String,
    op: Option<Ops>,
}

impl Node {
    pub fn new(name: impl ToString) -> Self {
        Self {
            id: AtomicId::new(),
            inputs: Vec::new(),
            name: name.to_string(),
            op: None,
        }
    }

    pub fn with_inputs(mut self, inputs: impl IntoIterator<Item = NodeIndex>) -> Self {
        self.inputs = Vec::from_iter(inputs);
        self
    }

    pub fn with_name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_op(mut self, op: impl Into<Ops>) -> Self {
        self.op = Some(op.into());
        self
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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn operation(&self) -> Option<&Ops> {
        self.op.as_ref()
    }
}

/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Node
//!
//!
//! The edges connecting to any given node are considered to be inputs and help to determine the flow of information
use crate::prelude::Ops;
use petgraph::prelude::NodeIndex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Node<T> {
    inputs: Vec<NodeIndex>,
    operation: Option<Ops>,
    value: Option<T>,
}

impl<T> Node<T> {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            operation: None,
            value: None,
        }
    }

    pub fn with_inputs(mut self, inputs: Vec<NodeIndex>) -> Self {
        self.inputs = inputs;
        self
    }

    pub fn with_op(mut self, operation: Ops) -> Self {
        self.operation = Some(operation);
        self
    }

    pub fn with_value(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }

    pub fn inputs(&self) -> &[NodeIndex] {
        &self.inputs
    }

    pub fn operation(&self) -> Option<&Ops> {
        self.operation.as_ref()
    }

    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }
}

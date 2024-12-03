/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Node
//!
//! A computational graph relies on weighted nodes to represent constants, operations, and variables.
//! The edges connecting to any given node are considered to be inputs and help to determine the flow of information
use crate::NodeIndex;
use rsdiff::id::AtomicId;
use rsdiff::ops::{Op, Operator};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, VariantNames};

pub trait ScgNode {
    fn id(&self) -> AtomicId;
    fn name(&self) -> &str;
}

macro_rules! impl_scg_node {
    ($($ty:ty),*) => {
        $(
            impl ScgNode for $ty {
                fn id(&self) -> AtomicId {
                    self.id
                }

                fn name(&self) -> &str {
                    &self.name
                }
            }
        )*
    };

}

impl_scg_node!(Placeholder, Operation);

#[derive(
    Clone,
    Debug,
    Display,
    EnumCount,
    EnumIs,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    SmartDefault,
    VariantNames,
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize,))]
pub enum Node {
    Operation(Operation),
    #[default]
    Placeholder(Placeholder),
}

impl Node {
    pub fn operation(inputs: impl IntoIterator<Item = NodeIndex>, op: impl Into<Op>) -> Self {
        Node::Operation(Operation::new(inputs, op))
    }

    pub fn placeholder(name: impl ToString) -> Self {
        Node::Placeholder(Placeholder::new(name))
    }

    pub fn id(&self) -> AtomicId {
        match self {
            Node::Operation(op) => op.id(),
            Node::Placeholder(ph) => ph.id(),
        }
    }

    pub fn inputs(&self) -> Option<&[NodeIndex]> {
        match self {
            Node::Operation(op) => Some(op.inputs()),
            _ => None,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Node::Operation(op) => op.name(),
            Node::Placeholder(ph) => ph.name(),
        }
    }

    pub fn op(&self) -> Option<&Op> {
        match self {
            Node::Operation(op) => Some(op.operation()),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize,))]
pub struct Placeholder {
    id: AtomicId,
    name: String,
}

impl Placeholder {
    pub fn new(name: impl ToString) -> Self {
        Self {
            id: AtomicId::new(),
            name: name.to_string(),
        }
    }

    pub fn with_name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }

    pub const fn id(&self) -> AtomicId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize,))]
pub struct Operation {
    id: AtomicId,
    inputs: Vec<NodeIndex>,
    name: String,
    op: Op,
}

impl Operation {
    pub fn new(inputs: impl IntoIterator<Item = NodeIndex>, op: impl Into<Op>) -> Self {
        let op = op.into();
        Self {
            id: AtomicId::new(),
            inputs: Vec::from_iter(inputs),
            name: op.name().to_string(),
            op,
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

    pub fn clear(&mut self) {
        self.inputs.clear();
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

    pub fn operation(&self) -> &Op {
        &self.op
    }
}

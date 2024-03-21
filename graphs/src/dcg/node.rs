/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use acme::ops::Operations;
use petgraph::prelude::NodeIndex;

#[derive(Clone, Debug)]
pub enum Node<T> {
    Op {
        inputs: Vec<NodeIndex>,
        op: Operations,
    },
    Input {
        param: bool,
        value: T,
    },
}

impl<T> Node<T> {
    pub fn op(inputs: impl IntoIterator<Item = NodeIndex>, op: impl Into<Operations>) -> Self {
        Node::Op {
            inputs: Vec::from_iter(inputs),
            op: op.into(),
        }
    }

    pub fn input(param: bool, value: T) -> Self {
        Node::Input { param, value }
    }

    pub fn get_value(&self) -> T
    where
        T: Copy + Default,
    {
        match self {
            Node::Input { value, .. } => *value,
            _ => T::default(),
        }
    }
}

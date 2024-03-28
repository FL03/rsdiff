/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::{BinaryExpr, Operations};
use crate::NodeIndex;

#[derive(Clone, Debug)]
pub enum Node<T> {
    Binary {
        lhs: NodeIndex,
        rhs: NodeIndex,
        op: BinaryExpr,
    },
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
    pub fn binary(lhs: NodeIndex, rhs: NodeIndex, op: impl Into<BinaryExpr>) -> Self {
        Node::Binary {
            lhs,
            rhs,
            op: op.into(),
        }
    }
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

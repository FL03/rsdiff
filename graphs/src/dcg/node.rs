/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::NodeIndex;
use rsdiff::id::AtomicId;
use rsdiff::ops::{BinaryOp, Op, UnaryOp};

#[derive(Clone, Debug)]
pub enum Node<T> {
    Binary {
        lhs: NodeIndex,
        rhs: NodeIndex,
        op: BinaryOp,
    },
    Unary {
        input: NodeIndex,
        op: UnaryOp,
    },
    Op {
        inputs: Vec<NodeIndex>,
        op: Op,
    },
    Input {
        id: AtomicId,
        param: bool,
        value: T,
    },
}

impl<T> Node<T> {
    pub fn binary(lhs: NodeIndex, rhs: NodeIndex, op: impl Into<BinaryOp>) -> Self {
        Node::Binary {
            lhs,
            rhs,
            op: op.into(),
        }
    }

    pub fn unary(input: NodeIndex, op: impl Into<UnaryOp>) -> Self {
        Node::Unary {
            input,
            op: op.into(),
        }
    }

    pub fn op(inputs: impl IntoIterator<Item = NodeIndex>, op: impl Into<Op>) -> Self {
        Node::Op {
            inputs: Vec::from_iter(inputs),
            op: op.into(),
        }
    }

    pub fn input(param: bool, value: T) -> Self {
        Node::Input {
            id: AtomicId::new(),
            param,
            value,
        }
    }

    pub fn value(&self) -> T
    where
        T: Copy + Default,
    {
        match self {
            Node::Input { value, .. } => *value,
            _ => T::default(),
        }
    }
}

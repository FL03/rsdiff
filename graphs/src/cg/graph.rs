/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{CGraph, Edge, Node};
use crate::NodeIndex;
use rsdiff::ops::{BinaryOp, Op, UnaryOp};

pub struct Graph<T> {
    store: CGraph<T>,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Self {
            store: CGraph::new(),
        }
    }

    pub fn add_edge(&mut self, src: NodeIndex, to: NodeIndex, edge: Edge) {
        self.store.add_edge(src, to, edge);
    }

    pub fn add_node(&mut self, weight: Node<T>) -> NodeIndex {
        self.store.add_node(weight)
    }

    pub fn add_node_data(&mut self, weight: T) -> NodeIndex {
        self.add_node(Node::new(weight, false))
    }

    pub fn add_node_param(&mut self, weight: T) -> NodeIndex {
        self.add_node(Node::new(weight, true))
    }

    pub fn op(&mut self, args: Vec<NodeIndex>, res: T, op: impl Into<Op>) -> NodeIndex {
        let dest = self.add_node_data(res);
        let edge = Edge::new(args.clone(), op);
        for arg in args {
            self.add_edge(dest, arg, edge.clone());
        }
        dest
    }
}

macro_rules! binary_op {
    ($($($p:ident)::*.$call:ident),*) => {
        $(
            binary_op!(@impl $($p)::*.$call);
        )*
    };
    (std $($p:ident.$call:ident),*) => {
        $(
            binary_op!(@impl core::ops::$p.$call);
        )*
    };
    (@impl $($p:ident)::*.$call:ident) => {
        pub fn $call(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex where T: $($p)::*<T, Output = T> {
            let (a, b) = get!(self[lhs, rhs]);

            let res = $($p)::*::$call(*a.data(), *b.data());
            self.op(vec![lhs, rhs], res, BinaryOp::$call())
        }
    };
}

macro_rules! unary_op {
    ($($($p:ident)::*.$call:ident$(($($rest:tt)*))?),* $(,)?) => {
        $(
            unary_op!(@impl $($p)::*.$call$(($($rest)*))?);

        )*
    };
    (core $($p:ident.$call:ident$(($($rest:tt)*))?),* $(,)?) => {
        $(
            unary_op!(@impl core::ops::$p.$call(where T: core::ops::$p<Output = T>));

        )*
    };
    (@impl $($p:ident)::*.$call:ident$(($($rest:tt)*))?) => {
        pub fn $call(&mut self, recv: NodeIndex) -> NodeIndex $($($rest)*)? {
            let (a,) = get!(self[recv]);

            let res = $($p)::*::$call(*a.data());
            self.op(vec![recv], res, UnaryOp::$call())
        }
    };
}

impl<T> Graph<T>
where
    T: Copy,
{
    binary_op!(std Add.add, Div.div, Mul.mul, Rem.rem, Sub.sub);

    binary_op!(num::traits::Pow.pow);

    unary_op!(core Neg.neg(), Not.not());
}

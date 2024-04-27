/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{CGraph, Edge, Node};
use crate::NodeIndex;
use acme::ops::{BinaryOp, Op};
use num::traits::{Num, Pow};

macro_rules! get {
    ($self:ident[$($index:expr),*]) => {
        (
            $(
                get!(@impl $self[$index]),
            )*
        )
    };
    (@impl $self:ident[$index:expr]) => {
        &$self.store[$index]
    };

}

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

    pub fn add_op(&mut self, args: Vec<NodeIndex>, res: T, op: impl Into<Op>) {
        let dest = self.add_node_data(res);
        let edge = Edge::new(args.clone(), op);
        for arg in args {
            self.add_edge(dest, arg, edge.clone());
        }
    }
}

impl<T> Graph<T>
where
    T: Copy + Num + Pow<T, Output = T>,
{
    pub fn add(&mut self, lhs: NodeIndex, rhs: NodeIndex) {
        let (a, b) = get!(self[lhs, rhs]);

        let res = *a.data() + *b.data();
        self.add_op(vec![lhs, rhs], res, BinaryOp::add());
    }

    pub fn div(&mut self, lhs: NodeIndex, rhs: NodeIndex) {
        let (a, b) = get!(self[lhs, rhs]);

        let res = *a.data() / *b.data();
        self.add_op(vec![lhs, rhs], res, BinaryOp::div());
    }

    pub fn mul(&mut self, lhs: NodeIndex, rhs: NodeIndex) {
        let (a, b) = get!(self[lhs, rhs]);

        let res = *a.data() * *b.data();
        self.add_op(vec![lhs, rhs], res, BinaryOp::mul());
    }

    pub fn pow(&mut self, lhs: NodeIndex, rhs: NodeIndex) {
        let (a, b) = get!(self[lhs, rhs]);

        let res = a.data().pow(*b.data());
        self.add_op(vec![lhs, rhs], res, BinaryOp::pow());
    }

    pub fn rem(&mut self, lhs: NodeIndex, rhs: NodeIndex) {
        let (a, b) = get!(self[lhs, rhs]);

        let res = *a.data() % *b.data();
        self.add_op(vec![lhs, rhs], res, BinaryOp::rem());
    }

    pub fn sub(&mut self, lhs: NodeIndex, rhs: NodeIndex) {
        let (a, b) = get!(self[lhs, rhs]);

        let res = *a.data() - *b.data();
        self.add_op(vec![lhs, rhs], res, BinaryOp::sub());
    }
}

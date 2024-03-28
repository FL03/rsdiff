/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::edge::Edge;
use super::node::Node;
use super::DynamicGraph;
use crate::ops::*;
use crate::prelude::GraphResult as Result;
use crate::NodeIndex;
use num::traits::NumAssign;
use petgraph::algo::toposort;
use std::collections::HashMap;
use std::ops::{Index, Neg};

pub struct Dcg<T> {
    store: DynamicGraph<T>,
}

impl<T> Dcg<T> {
    pub fn new() -> Self {
        Dcg {
            store: DynamicGraph::new(),
        }
    }

    pub fn binary(
        &mut self,
        lhs: NodeIndex,
        rhs: NodeIndex,
        op: impl Into<BinaryExpr>,
    ) -> NodeIndex {
        let c = self.store.add_node(Node::binary(lhs, rhs, op));
        self.store.add_edge(lhs, c, Edge::new(lhs));
        self.store.add_edge(rhs, c, Edge::new(rhs));
        c
    }

    pub fn constant(&mut self, value: T) -> NodeIndex {
        self.input(false, value)
    }

    pub fn get(&self, index: NodeIndex) -> Option<&Node<T>> {
        self.store.node_weight(index)
    }

    pub fn include(&mut self, node: impl Into<Node<T>>) -> NodeIndex {
        self.store.add_node(node.into())
    }

    pub fn input(&mut self, param: bool, value: T) -> NodeIndex {
        self.store.add_node(Node::input(param, value))
    }

    pub fn op(
        &mut self,
        inputs: impl IntoIterator<Item = NodeIndex>,
        op: impl Into<Operations>,
    ) -> NodeIndex {
        let args = Vec::from_iter(inputs);

        let c = self.store.add_node(Node::op(args.clone(), op));
        for arg in args {
            self.store.add_edge(arg, c, Edge::new(arg));
        }
        c
    }

    pub fn remove(&mut self, index: NodeIndex) -> Option<Node<T>> {
        self.store.remove_node(index)
    }

    pub fn unary(&mut self, input: NodeIndex, op: impl Into<UnaryExpr>) -> NodeIndex {
        let c = self.store.add_node(Node::unary(input, op));
        self.store.add_edge(input, c, Edge::new(input));
        c
    }

    pub fn variable(&mut self, value: T) -> NodeIndex {
        self.input(true, value)
    }
}

impl<T> Dcg<T> {
    pub fn add(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        self.binary(lhs, rhs, BinaryExpr::add())
    }

    pub fn div(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        self.binary(lhs, rhs, BinaryExpr::div())
    }

    pub fn mul(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        self.binary(lhs, rhs, BinaryExpr::mul())
    }

    pub fn sub(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        self.binary(lhs, rhs, BinaryExpr::sub())
    }
}

impl<T> Dcg<T>
where
    T: Copy + Default + Neg<Output = T> + NumAssign,
{
    pub fn backward(&self) -> Result<HashMap<NodeIndex, T>> {
        let sorted = toposort(&self.store, None)?;
        let target = sorted.last().unwrap();
        self.gradient(*target)
    }
    pub fn gradient(&self, target: NodeIndex) -> Result<HashMap<NodeIndex, T>> {
        let mut store = HashMap::<NodeIndex, T>::new();
        // initialize the stack
        let mut stack = Vec::<(NodeIndex, T)>::new();
        // start by computing the gradient of the target w.r.t. itself
        stack.push((target, T::one()));
        store.insert(target, T::one());

        while let Some((i, grad)) = stack.pop() {
            let node = &self[i];

            match node {
                Node::Binary { lhs, rhs, op } => match op {
                    BinaryExpr::Add(_) => {
                        *store.entry(*lhs).or_default() += grad;
                        *store.entry(*rhs).or_default() += grad;

                        stack.push((*lhs, grad));
                        stack.push((*rhs, grad));
                    }
                    BinaryExpr::Mul(_) => {
                        let lhs_grad = grad * self[*rhs].value();
                        let rhs_grad = grad * self[*lhs].value();
                        *store.entry(*lhs).or_default() += lhs_grad;
                        *store.entry(*rhs).or_default() += rhs_grad;

                        stack.push((*lhs, lhs_grad));
                        stack.push((*rhs, rhs_grad));
                    }
                    BinaryExpr::Sub(_) => {
                        *store.entry(*lhs).or_default() += grad;
                        *store.entry(*rhs).or_default() -= grad;

                        stack.push((*lhs, grad));
                        stack.push((*rhs, grad.neg()));
                    }
                    _ => {}
                },
                Node::Unary { op, .. } => match op {
                    _ => {
                        unimplemented!();
                    }
                },
                Node::Input { param, .. } => {
                    if *param {
                        continue;
                    }
                    *store.entry(i).or_default() += grad;
                    stack.push((i, grad));
                }
                _ => {}
            }
        }

        Ok(store)
    }
}

impl<T> Index<NodeIndex> for Dcg<T> {
    type Output = Node<T>;

    fn index(&self, index: NodeIndex) -> &Self::Output {
        self.get(index).unwrap()
    }
}

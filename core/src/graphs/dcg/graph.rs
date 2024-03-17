/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::edge::Edge;
use super::node::Node;
use super::DynamicGraph;
use crate::ops::*;
use crate::prelude::Result;
use num::traits::{Num, NumAssignOps, NumOps};
use petgraph::algo::toposort;
use petgraph::prelude::{Direction, NodeIndex};
use std::collections::HashMap;
use std::ops::Index;

pub struct Dcg<T> {
    store: DynamicGraph<T>,
}

impl<T> Dcg<T> {
    pub fn new() -> Self {
        Dcg {
            store: DynamicGraph::new(),
        }
    }

    pub fn get(&self, index: NodeIndex) -> Option<&Node<T>> {
        self.store.node_weight(index)
    }

    pub fn include(&mut self, node: impl Into<Node<T>>) -> NodeIndex {
        self.store.add_node(node.into())
    }

    pub fn remove(&mut self, index: NodeIndex) -> Option<Node<T>> {
        self.store.remove_node(index)
    }

    pub fn input(&mut self, param: bool, value: T) -> NodeIndex {
        self.store.add_node(Node::input(param, value))
    }

    pub fn op(
        &mut self,
        inputs: impl IntoIterator<Item = NodeIndex>,
        op: impl Into<Ops>,
    ) -> NodeIndex {
        let args = Vec::from_iter(inputs);

        let c = self.store.add_node(Node::op(args.clone(), op));
        for arg in args {
            self.store.add_edge(arg, c, Edge::new(arg));
        }
        c
    }
}

impl<T> Dcg<T> {
    pub fn add(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        self.op([lhs, rhs], BinaryOp::add())
    }

    pub fn mul(&mut self, lhs: NodeIndex, rhs: NodeIndex) -> NodeIndex {
        self.op([lhs, rhs], BinaryOp::mul())
    }

    pub fn backward(&self) -> Result<HashMap<NodeIndex, T>>
    where
        T: Copy + Default + Num + NumAssignOps + NumOps,
    {
        let sorted = toposort(&self.store, None)?;
        let target = *sorted.last().unwrap();

        let mut gradients = HashMap::<NodeIndex, T>::new();
        gradients.insert(target, T::one());

        for node in sorted.iter().rev() {
            let node_grad = gradients[node];
            let node_op = self.get(*node).unwrap();

            if let Node::Op { inputs, op } = node_op {
                match op {
                    Ops::Binary(inner) => match *inner {
                        BinaryOp::Add(_) => {
                            for arg in self.store.neighbors_directed(*node, Direction::Incoming) {
                                *gradients.entry(arg).or_default() += node_grad;
                            }
                        }
                        BinaryOp::Mul(_) => {
                            let lhs = inputs[0];
                            let rhs = inputs[1];
                            let lhs_val = self.get(lhs).unwrap().get_value();
                            let rhs_val = self.get(rhs).unwrap().get_value();
                            *gradients.entry(lhs).or_default() += node_grad * rhs_val;
                            *gradients.entry(rhs).or_default() += node_grad * lhs_val;
                        }
                        _ => {}
                    },
                    // Handle other operations as needed
                    _ => {}
                }
            }
        }

        Ok(gradients)
    }

    pub fn gradient(&self, output: NodeIndex) -> Result<HashMap<NodeIndex, T>>
    where
        T: Copy + Default + Num + NumAssignOps + NumOps,
    {
        let mut gradients = HashMap::<NodeIndex, T>::new();
        gradients.insert(output, T::one()); // Initialize output gradient to 1.0

        let topo = toposort(&self.store, None)?;

        for node in topo.iter().rev() {
            let node_grad = gradients[node];
            let node_op = self.get(*node).unwrap();

            if let Node::Op { inputs, op } = node_op {
                match op {
                    Ops::Binary(BinaryOp::Add(_)) => {
                        for arg in self.store.neighbors_directed(*node, Direction::Incoming) {
                            *gradients.entry(arg).or_default() += node_grad;
                        }
                    }
                    Ops::Binary(BinaryOp::Mul(_)) => {
                        let lhs = inputs[0];
                        let rhs = inputs[1];
                        let lhs_val = self[lhs].get_value();
                        let rhs_val = self[rhs].get_value();
                        *gradients.entry(lhs).or_default() += node_grad * rhs_val;
                        *gradients.entry(rhs).or_default() += node_grad * lhs_val;
                    }
                    // Handle other operations as needed
                    _ => {}
                }
            }
        }

        Ok(gradients)
    }
}

impl<T> Index<NodeIndex> for Dcg<T> {
    type Output = Node<T>;

    fn index(&self, index: NodeIndex) -> &Self::Output {
        self.get(index).unwrap()
    }
}

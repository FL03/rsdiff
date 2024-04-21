/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Edge, Node, Operation};
use crate::prelude::GraphResult as Result;
use acme::ops::{Arithmetic, BinaryOp, Op, UnaryOp};
use num::traits::{NumAssign, NumOps, Signed};
use petgraph::algo::toposort;
use petgraph::prelude::{DiGraph, NodeIndex};
use std::collections::BTreeMap;

pub(crate) type ValueStore<T> = BTreeMap<NodeIndex, T>;

#[derive(Clone, Debug)]
pub struct Scg<T> {
    graph: DiGraph<Node, Edge<T>>,
    vals: ValueStore<T>,
}

impl<T> Default for Scg<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Scg<T> {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            vals: BTreeMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.graph.clear();
    }

    pub fn get(&self, index: NodeIndex) -> Option<&Node> {
        self.graph.node_weight(index)
    }

    pub fn get_value(&self, index: NodeIndex) -> Option<&T> {
        self.vals.get(&index)
    }

    pub fn constant(&mut self, name: impl ToString, data: T) -> NodeIndex {
        let v = self.graph.add_node(Node::placeholder(name));
        self.vals.insert(v, data);
        v
    }

    pub fn operation(
        &mut self,
        inputs: impl IntoIterator<Item = NodeIndex>,
        operation: impl Into<Op>,
        result: Option<T>,
    ) -> Result<NodeIndex>
    where
        T: Default,
    {
        let op = Operation::new(inputs, operation);
        let node = Node::Operation(op.clone());
        let v = self.graph.add_node(node.clone());
        let _ = self.vals.insert(v, result.unwrap_or_default());
        Ok(v)
    }

    pub fn variable(&mut self, value: T) -> NodeIndex {
        let v = self.graph.add_node(Node::default());
        self.vals.insert(v, value);
        v
    }
}

impl<T> Scg<T>
where
    T: Copy + Default + NumAssign + NumOps + Signed + 'static,
{
    pub fn backward(&self) -> Result<ValueStore<T>> {
        // find the topological order of the graph
        let nodes: Vec<NodeIndex> = toposort(&self.graph, None)?;
        // compute the gradient w.r.t. the last topological node
        self.gradient_at(*nodes.last().unwrap())
    }

    pub fn gradient_at(&self, target: NodeIndex) -> Result<ValueStore<T>> {
        // initialize the gradient store
        let mut gradients = ValueStore::new();
        // initialize the stack
        let mut stack = Vec::<(NodeIndex, T)>::new();
        // start by computing the gradient of the target w.r.t. itself
        gradients.insert(target, T::one());
        stack.push((target, T::one()));
        // iterate through the nodes in reverse topological order
        while let Some((i, grad)) = stack.pop() {
            // get the current node
            let node = &self.graph[i];
            if let Some(inputs) = node.inputs() {
                if inputs.is_empty() {
                    continue;
                }
                // iterate through the inputs of the current node
                for (j, input) in inputs.iter().enumerate() {
                    // calculate the gradient of each input w.r.t. the current node
                    let dt = if let Some(op) = node.op() {
                        match op {
                            Op::Binary(base) => match base {
                                BinaryOp::Arith(inner) => match inner {
                                    Arithmetic::Add(_) => grad,
                                    Arithmetic::Div(_) => {
                                        let out = self.vals[&i];
                                        let val = self.vals[input];
                                        if j % 2 == 0 {
                                            grad / val
                                        } else {
                                            -grad * out / (val * val)
                                        }
                                    }
                                    Arithmetic::Mul(_) => {
                                        let out = self.vals[&i];
                                        let val = self.vals[input];
                                        grad * out / val
                                    }
                                    Arithmetic::Sub(_) => {
                                        if j % 2 == 0 {
                                            grad
                                        } else {
                                            grad.neg()
                                        }
                                    }
                                    _ => todo!(),
                                },
                                _ => todo!(),
                            },
                            Op::Unary(base) => match base {
                                UnaryOp::Neg => -grad,
                                _ => todo!(),
                            },
                            _ => todo!(),
                        }
                    } else {
                        T::default()
                    };
                    // add or insert the gradient of the input
                    *gradients.entry(*input).or_default() += dt;
                    // push the input and its gradient onto the stack
                    stack.push((*input, dt));
                }
            }
        }
        Ok(gradients)
    }
}

impl<T> Scg<T>
where
    T: Copy + Default + NumOps + PartialOrd,
{
    pub fn add(&mut self, a: NodeIndex, b: NodeIndex) -> Result<NodeIndex> {
        let x = self.vals[&a];
        let y = self.vals[&b];
        let op = BinaryOp::add();
        let res = x + y;

        let c = self.operation([a, b], op, Some(res))?;
        Ok(c)
    }

    pub fn mul(&mut self, a: NodeIndex, b: NodeIndex) -> Result<NodeIndex> {
        let x = self.vals[&a];
        let y = self.vals[&b];
        let res = x * y;
        let c = self.operation([a, b], BinaryOp::mul(), Some(res))?;

        Ok(c)
    }
}

/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Node;
use crate::prelude::{BinaryOp, BinaryOperation, Ops, Result};
use num::traits::{NumAssign, NumOps, Signed};
use petgraph::algo::toposort;
use petgraph::prelude::{DiGraph, NodeIndex};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Scg<T> {
    graph: DiGraph<Node, usize>,
    vals: HashMap<NodeIndex, T>,
}

impl<T> Scg<T> {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            vals: HashMap::new(),
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

    pub fn constant(&mut self, value: T) -> NodeIndex {
        let v = self.graph.add_node(Node::default());
        self.vals.insert(v, value);
        v
    }

    pub fn operation(
        &mut self,
        inputs: impl IntoIterator<Item = NodeIndex>,
        operation: impl Into<Ops>,
        result: Option<T>,
    ) -> Result<NodeIndex>
    where
        T: Default,
    {
        let node = Node::default().with_inputs(inputs).with_op(operation);
        let v = self.graph.add_node(node.clone());
        let edges = node.inputs().iter().map(|i| (*i, v));
        let _val = self.vals.insert(v, result.unwrap_or_default());
        self.graph.extend_with_edges(edges);
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
    pub fn backward(&self) -> Result<HashMap<NodeIndex, T>> {
        // find the topological order of the graph
        let nodes: Vec<NodeIndex> = toposort(&self.graph, None)?;
        // compute the gradient w.r.t. the last topological node
        self.gradient_at(nodes.last().unwrap().clone())
    }

    pub fn gradient_at(&self, target: NodeIndex) -> Result<HashMap<NodeIndex, T>> {
        // initialize the gradient store
        let mut gradients = HashMap::new();
        // initialize the stack
        let mut stack = Vec::<(NodeIndex, T)>::new();
        // start by computing the gradient of the target w.r.t. itself
        gradients.insert(target, T::one());
        stack.push((target, T::one()));
        // iterate through the nodes in reverse topological order
        while let Some((i, grad)) = stack.pop() {
            // get the current node
            let node = &self.graph[i];
            // iterate through the inputs of the current node
            for (j, input) in node.inputs().iter().enumerate() {
                // calculate the gradient of each input w.r.t. the current node
                let dt = if let Some(op) = node.operation() {
                    match op {
                        Ops::Binary(op) => match op {
                            BinaryOp::Add(_) => grad,
                            BinaryOp::Div(_) => {
                                let out = self.vals[&i];
                                let val = self.vals[input];
                                if j % 2 == 0 {
                                    grad / val
                                } else {
                                    -grad * out / (val * val)
                                }
                            }
                            BinaryOp::Mul(_) => {
                                let out = self.vals[&i];
                                let val = self.vals[input];
                                grad * out / val
                            }
                            BinaryOp::Sub(_) => {
                                if j % 2 == 0 {
                                    grad
                                } else {
                                    grad.neg()
                                }
                            }
                            _ => T::default(),
                        },
                        _ => T::default(),
                    }
                } else {
                    T::default()
                };
                // add or insert the gradient of the input
                *gradients.entry(*input).or_insert(T::default()) += dt;
                // push the input and its gradient onto the stack
                stack.push((*input, dt));
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
        let res = op.eval(x, y);

        let c = self.operation([a, b], op, Some(res))?;
        Ok(c)
    }

    pub fn div(&mut self, a: NodeIndex, b: NodeIndex) -> Result<NodeIndex> {
        let x = self.vals[&a];
        let y = self.vals[&b];
        let res = x / y;

        let op = BinaryOp::div();
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

    pub fn sub(&mut self, a: NodeIndex, b: NodeIndex) -> Result<NodeIndex> {
        let x = self.vals[&a];
        let y = self.vals[&b];
        let res = x - y;
        let c = self.operation([a, b], BinaryOp::sub(), Some(res))?;

        Ok(c)
    }
}

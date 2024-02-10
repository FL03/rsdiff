/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Node;
use crate::prelude::{BinaryOp, Ops, Result};
use daggy::petgraph::algo::toposort;
use daggy::{Dag, NodeIndex};
use num::traits::{NumAssign, NumOps, Signed};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Graph<T> {
    graph: Dag<Node, usize>,
    vals: HashMap<NodeIndex, T>,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Self {
            graph: Dag::new(),
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
        let v = self.graph.add_node(Node::new(vec![], None));
        self.vals.insert(v, value);
        v
    }

    pub fn operation(
        &mut self,
        inputs: impl IntoIterator<Item = NodeIndex>,
        operation: Ops,
        result: Option<T>,
    ) -> Result<NodeIndex>
    where
        T: Default,
    {
        let node = Node::new(inputs, Some(operation));
        let v = self.graph.add_node(node.clone());
        let edges = node.inputs().iter().map(|i| (*i, v));
        let _val = self.vals.insert(v, result.unwrap_or_default());
        self.graph.extend_with_edges(edges)?;
        Ok(v)
    }

    pub fn variable(&mut self, value: T) -> NodeIndex {
        let v = self.graph.add_node(Node::new(vec![], None));
        self.vals.insert(v, value);
        v
    }
}

impl<T> Graph<T>
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
        // initialize the gradients
        gradients.insert(target, T::one());
        stack.push((target, T::one()));
        // iterate through the nodes in reverse topological order
        while let Some((i, grad)) = stack.pop() {
            // get the current node
            let node = self.graph[i].clone();
            // iterate through the inputs of the current node
            for (j, input) in node.inputs().iter().enumerate() {
                // calculate the gradient of each input w.r.t. the current node
                let dt = if let Some(op) = node.operation() {
                    match op {
                        Ops::Binary(op) => match op {
                            BinaryOp::Add => grad,
                            BinaryOp::Div => {
                                let out = self.vals[&i];
                                let val = self.vals[input];
                                if j % 2 == 0 {
                                    grad / val
                                } else {
                                    -grad * out / (val * val)
                                }
                            }
                            BinaryOp::Mul => {
                                let out = self.vals[&i];
                                let val = self.vals[input];
                                grad * out / val
                            }
                            BinaryOp::Sub => {
                                if j % 2 == 0 {
                                    grad
                                } else {
                                    -grad
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

impl<T> Graph<T>
where
    T: Copy + Default + NumOps,
{
    pub fn add(&mut self, a: NodeIndex, b: NodeIndex) -> Result<NodeIndex> {
        let x = self.vals[&a];
        let y = self.vals[&b];
        let res = x + y;

        let c = self.operation([a, b], BinaryOp::Add.into(), Some(res))?;

        Ok(c)
    }

    pub fn div(&mut self, a: NodeIndex, b: NodeIndex) -> Result<NodeIndex> {
        let x = self.vals[&a];
        let y = self.vals[&b];
        let res = x / y;
        let c = self.operation([a, b], BinaryOp::Div.into(), Some(res))?;

        Ok(c)
    }

    pub fn mul(&mut self, a: NodeIndex, b: NodeIndex) -> Result<NodeIndex> {
        let x = self.vals[&a];
        let y = self.vals[&b];
        let res = x * y;
        let c = self.operation([a, b], BinaryOp::Mul.into(), Some(res))?;

        Ok(c)
    }

    pub fn sub(&mut self, a: NodeIndex, b: NodeIndex) -> Result<NodeIndex> {
        let x = self.vals[&a];
        let y = self.vals[&b];
        let res = x - y;
        let c = self.operation([a, b], BinaryOp::Sub.into(), Some(res))?;

        Ok(c)
    }
}

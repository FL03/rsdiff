/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Node;
use crate::prelude::Result;
use daggy::petgraph::algo::toposort;
use daggy::{Dag, NodeIndex};
use num::traits::{NumAssign, NumOps};
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
        let v = self.graph.add_node(Node::new(vec![], "constant"));
        self.vals.insert(v, value);
        v
    }

    pub fn operation(
        &mut self,
        inputs: impl IntoIterator<Item = NodeIndex>,
        operation: impl ToString,
        result: Option<T>,
    ) -> Result<NodeIndex>
    where
        T: Default,
    {
        let node = Node::new(inputs, operation);
        let v = self.graph.add_node(node.clone());
        let edges = node.inputs().iter().map(|i| (*i, v));
        self.vals.insert(v, result.unwrap_or_default());
        self.graph.extend_with_edges(edges)?;
        Ok(v)
    }

    pub fn variable(&mut self, value: T) -> NodeIndex {
        let v = self.graph.add_node(Node::new(vec![], "var"));
        self.vals.insert(v, value);
        v
    }
}

impl<T> Graph<T>
where
    T: Copy + Default + NumAssign + NumOps + 'static,
{
    pub fn backward(&self) -> Result<HashMap<NodeIndex, T>> {
        // find the topological order of the graph
        let nodes: Vec<NodeIndex> = toposort(&self.graph, None)?.iter().rev().cloned().collect();
        // initialize the gradient store
        let mut gradients = HashMap::new();
        let mut stack = Vec::new();
        // initialize the gradients
        gradients.insert(nodes.first().unwrap().clone(), T::one());
        stack.push((nodes.first().unwrap().clone(), T::one()));
        // iterate through the nodes in reverse topological order
        while let Some((i, grad)) = stack.pop() {
            // get the current node
            let node = self.graph[i].clone();
            // iterate through the inputs of the current node
            for input in node.inputs() {
                // calculate the gradient of each input w.r.t. the current node
                let dt = match node.operation() {
                    "add" => grad,
                    "mul" => {
                        let out = self.vals[&i];
                        let val = self.vals[input];
                        grad * out / val
                    }
                    _ => T::default(),
                };
                *gradients.entry(*input).or_insert(T::default()) += dt;
                stack.push((*input, dt));
            }
        }
        Ok(gradients)
    }
    pub fn gradient_at(&mut self, target: NodeIndex) -> Result<HashMap<NodeIndex, T>> {
        let mut gradients = HashMap::new();
        let mut stack = Vec::new();

        gradients.insert(target, T::one());
        stack.push((target, T::one()));
        while let Some((i, grad)) = stack.pop() {
            let node = self.graph[i].clone();

            for input in node.inputs() {
                let dt = match node.operation() {
                    "add" => grad,
                    "mul" => {
                        let out = self.vals[&i];
                        let val = self.vals[input];
                        grad * out / val
                    }
                    _ => T::default(),
                };
                *gradients.entry(*input).or_insert(T::default()) += dt;
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

        let c = self.operation([a, b], "add", Some(res))?;

        Ok(c)
    }

    pub fn mul(&mut self, a: NodeIndex, b: NodeIndex) -> Result<NodeIndex> {
        let x = self.vals[&a];
        let y = self.vals[&b];
        let res = x * y;
        let c = self.operation([a, b], "mul", Some(res))?;

        Ok(c)
    }
}

/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Config, Node,};
use crate::prelude::Result;
use crate::stores::{GradientStore, Store};
use daggy::petgraph::algo::toposort;
use daggy::{Dag, NodeIndex};
use num::traits::{NumAssign, NumOps};
use std::collections::HashMap;

#[derive(Clone)]
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

    pub fn variable(&mut self, value: T) -> NodeIndex {
        let v = self.graph.add_node(Node::new(vec![], "input"));
        self.vals.insert(v, value);
        v
    }
}

impl<T> Graph<T>
where
    T: Copy + Default + NumAssign + NumOps + 'static,
{
    pub fn gradient_at(&mut self, target: NodeIndex) -> Result<HashMap<NodeIndex, T>> {
        let graph = self.clone();
        let nodes = toposort(&self.graph, None)?;

        let mut gradients = HashMap::new();
        gradients.insert(target, *self.get_value(target).unwrap_or(&T::one()));
        for i in nodes.iter().rev() {
            let node = graph.get(*i).unwrap_or(&Node::default()).clone();
            let grad = *gradients.get(&i).unwrap_or(&T::default());
            for input in node.inputs() {
                let dt = match node.operation() {
                    "add" => {
                        grad
                    },
                    "mul" => {
                        let x = *graph.get_value(*i).unwrap();
                        let out = *graph.get_value(*input).unwrap();
                        grad * x / out
                    },
                    _ => T::default()
                };
                *gradients.entry(*input).or_insert(T::default()) += dt;
            }
            
            
        }
        Ok(gradients)
    }
}

impl<T> Graph<T>
where
    T: Clone + Default + NumOps,
{
    pub fn add(&mut self, a: NodeIndex, b: NodeIndex) -> Result<NodeIndex> {
        let x = self.get_value(a).unwrap().clone();
        let y = self.get_value(b).unwrap().clone();
        let res = x + y;

        let c = self.graph.add_node(Node::new(vec![a, b], "add"));
        self.vals.insert(c, res);
        let _ac = self.graph.add_edge(a, c, 0)?;
        let _bc = self.graph.add_edge(b, c, 0)?;

        Ok(c)
    }

    pub fn mul(&mut self, a: NodeIndex, b: NodeIndex) -> Result<NodeIndex> {
        let x = self.get_value(a).unwrap().clone();
        let y = self.get_value(b).unwrap().clone();
        let res = x * y;
        let c = self.graph.add_node(Node::new(vec![a, b], "mul"));
        self.vals.insert(c, res);
        let _ac = self.graph.add_edge(a, c, 0)?;
        let _bc = self.graph.add_edge(b, c, 0)?;

        // let fg = | graph: &mut dyn Arithmetic<NodeIndex>, store: &mut GradientStore, rhs: T | {
        //     //
        //     if let Some(grad) = store.get(&a) {
        //         let grad = graph.mul(*grad, b);
        //         store.add_gradient(self, a, &grad);
        //     }
        //     if let Some(grad) = store.get(&b) {
        //         let grad = graph.mul(*grad, a);
        //         store.add_gradient(self, b, &grad);
        //     }
        //     Ok(())
        // };
        Ok(c)
    }
}

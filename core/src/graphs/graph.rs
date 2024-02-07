/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Arithmetic;
use crate::cmp::FnNode;
use crate::exp::ops::Addition;
use crate::ops::Evaluate;
use crate::prelude::Result;
use crate::stores::{GradientStore, Store};
use daggy::petgraph::algo::toposort;
use daggy::{Dag, NodeIndex};
use num::traits::NumOps;
use std::collections::HashMap;

pub struct Graph<T> {
    graph: Dag<T, usize>,
    ops: HashMap<NodeIndex, String>,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Self {
            graph: Dag::new(),
            ops: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.graph.clear();
    }

    pub fn get(&self, index: NodeIndex) -> Option<&T> {
        self.graph.node_weight(index)
    }

    pub fn variable(&mut self, value: T) -> NodeIndex {
        self.graph.add_node(value)
    }
}

impl<T> Graph<T>
where
    T: Clone + Default + 'static,
{
    pub fn compute_gradients(&mut self, target: NodeIndex) -> Result<()> {
        let nodes = toposort(&self.graph, None)?;

        let mut gradients = GradientStore::new();
        gradients.insert(target, self.get(target).unwrap().clone());
        for i in nodes {
            let node = self.get(i).unwrap().clone();
            if let Some(op) = self.ops.get(&i) {
                match op.as_str() {
                    "add" => {
                        gradients.insert(i, node);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

impl<T> Arithmetic<NodeIndex> for Graph<T>
where
    T: Clone + Default + NumOps,
{
    fn add(&mut self, a: NodeIndex, b: NodeIndex) -> Result<NodeIndex> {
        let x = self.get(a).unwrap().clone();
        let y = self.get(b).unwrap().clone();
        let res = x + y;

        let c = self.graph.add_node(res);
        self.ops.insert(c, "add".to_string());
        self.graph
            .extend_with_edges([(a, c), (b, c)])
            .expect("Failed to add edge");

        Ok(c)
    }

    fn mul(&mut self, a: NodeIndex, b: NodeIndex) -> Result<NodeIndex> {
        let x = self.graph.node_weight(a).unwrap().clone();
        let y = self.graph.node_weight(b).unwrap().clone();
        let res = x * y;
        let c = self.graph.add_node(res);

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

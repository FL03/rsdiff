/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::Result;
use crate::stores::{GradientStore, Store};
use daggy::petgraph::algo::toposort;
use daggy::{Dag, NodeIndex};

pub struct Graph<T> {
    graph: Dag<T, usize>,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Self { graph: Dag::new() }
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
        Ok(())
    }
}

/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{DcgEdge, Node};
use crate::prelude::Result;
use crate::stores::{GradientStore, Store};
use petgraph::algo::toposort;
use petgraph::prelude::{DiGraph, NodeIndex};

pub struct Dcg<T> {
    graph: DiGraph<Node<T>, DcgEdge>,
}

impl<T> Dcg<T> {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
        }
    }

    pub fn clear(&mut self) {
        self.graph.clear();
    }

    pub fn get(&self, index: NodeIndex) -> Option<&Node<T>> {
        self.graph.node_weight(index)
    }

    pub fn variable(&mut self, value: T) -> NodeIndex {
        self.graph.add_node(Node::new().with_value(value))
    }
}

impl<T> Dcg<T>
where
    T: Clone + Default + 'static,
{
    pub fn compute_gradients(&mut self, target: NodeIndex) -> Result<()> {
        let nodes = toposort(&self.graph, None)?;

        let mut gradients = GradientStore::new();
        gradients.insert(target, self.graph[target].clone());
        Ok(())
    }
}

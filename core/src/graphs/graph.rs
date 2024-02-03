/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Arithmetic;
use crate::cmp::{Constant, Variable};
use crate::ops::Evaluate;
use crate::prelude::Result;
use crate::stores::{GradientStore, Store};
use daggy::petgraph::algo::toposort;
use daggy::{Dag, NodeIndex};
use num::traits::NumOps;
use std::collections::HashMap;

pub enum FnNode<T> {
    Const(Constant<T>),
    Var(Variable<T>),
    Op(T),
}

pub struct FnGraph<T> {
    graph: Dag<T, usize>,
}

impl<T> FnGraph<T> {
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

impl<T> FnGraph<T>
where
    T: Clone + Default + 'static,
{
    pub fn compute_gradients(&mut self, target: NodeIndex) -> Result<()> {
        let nodes = toposort(&self.graph, None)?;
        let mut gradients = GradientStore::new();
        gradients.insert(target, self.get(target).unwrap().clone());
        Ok(())
    }

    pub fn operator(
        &mut self,
        inputs: Vec<NodeIndex>,
        op: impl Evaluate<Vec<T>, Output = T>,
    ) -> NodeIndex {
        let args = inputs
            .iter()
            .map(|i| self.graph.node_weight(*i).unwrap())
            .cloned()
            .collect();
        let c = self.graph.add_node(op.eval(args));
        self.graph
            .extend_with_edges(inputs.into_iter().map(|i| (i, c)))
            .expect("Failed to add edge");
        c
    }
}

impl<T> Arithmetic<NodeIndex> for FnGraph<T>
where
    T: Clone + Default + NumOps,
{
    fn add(&mut self, a: NodeIndex, b: NodeIndex) -> NodeIndex {
        let x = self.graph.node_weight(a).unwrap().clone();
        let y = self.graph.node_weight(b).unwrap().clone();
        let res = x + y;
        let c = self.graph.add_node(res);
        self.graph
            .extend_with_edges([(a, c), (b, c)])
            .expect("Failed to add edge");

        c
    }

    fn mul(&mut self, a: NodeIndex, b: NodeIndex) -> NodeIndex {
        let x = self.graph.node_weight(a).unwrap().clone();
        let y = self.graph.node_weight(b).unwrap().clone();
        let res = x * y;
        let c = self.graph.add_node(res);

        let ac = self.graph.add_edge(a, c, 0).expect("Failed to add edge");
        let bc = self.graph.add_edge(b, c, 0).expect("Failed to add edge");

        let fg = |graph: &mut dyn Arithmetic<NodeIndex>, store: &mut GradientStore, rhs: T| {
            if let Some(grad) = store.get(&c) {
                let grad = graph.mul(*grad, b);
                store.add_gradient(self, a, &grad);
            }
            if let Some(grad) = store.get(&b) {
                let grad = graph.mul(*grad, a);
                store.add_gradient(self, b, &grad);
            }
        };
        c
    }
}

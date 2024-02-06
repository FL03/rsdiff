/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Addition, Config, GradientUpdater, Node};
use crate::ops::{BinaryOp, Op};
use crate::prelude::{Arithmetic, Evaluate, GradientStore, Result, Store, Variable};
use daggy::petgraph::algo::toposort;
use daggy::petgraph::visit::IntoEdges;
use daggy::{Dag, NodeIndex};
use num::traits::NumOps;
use std::collections::HashMap;
use std::sync::Arc;

pub struct FnGraph<C: Config> {
    graph: Dag<Node<C::DType>, Option<C::DType>>,
    gradients: HashMap<usize, Option<GradientUpdater<C>>>,
}

impl<C> FnGraph<C>
where
    C: Config,
{
    pub fn new() -> Self {
        Self {
            graph: Dag::new(),
            gradients: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.graph.clear();
        self.gradients.clear();
    }

    pub fn get(&self, index: NodeIndex) -> Option<&Node<<C as Config>::DType>> {
        self.graph.node_weight(index)
    }

    pub fn variable(
        &mut self,
        name: impl ToString,
        value: Option<<C as Config>::DType>,
    ) -> NodeIndex {
        let var = Variable::new(name, value);
        let node = Node::Var(var);
        self.graph.add_node(node)
    }
}

impl<C> FnGraph<C>
where
    C: Config,
    C::DType: Clone + Default + 'static,
{
    pub fn compute_gradients(&mut self, target: NodeIndex) -> Result<()> {
        // retrieve the topological order of the graph
        let nodes = toposort(&self.graph, None)?;
        // create a new gradient store
        let mut store = GradientStore::new();
        // retrieve the target node
        let node_t = self.get(target).unwrap().clone();
        // insert the target node into the store
        store.insert(target, node_t.clone());
        for i in nodes {
            if i == target {
                continue;
            }
            let node = self.get(i).unwrap().clone();
            match node {
                Node::Op { name, args } => {
                    let mut grad = C::DType::default();
                    match name.as_str() {
                        "add" => {
                            let a = self.get(args[0]).unwrap().clone();
                            let b = self.get(args[1]).unwrap().clone();
                            // grad = a + b;
                        }
                        _ => {}
                    }
                }
                _ => {
                    store.insert(i, C::DType::default());
                }
            }
        }
        Ok(())
    }
}

impl<C> FnGraph<C>
where
    C: Config<Store = GradientStore<usize>>,
    C::DType: Copy + Default + Evaluate<Output = Variable<C::DType>> + NumOps + 'static,
    C::Eval: Arithmetic<NodeIndex>,
{
    pub fn add(&mut self, a: NodeIndex, b: NodeIndex) -> Result<NodeIndex> {
        let x = self.graph.node_weight(a).unwrap().clone();
        let y = self.graph.node_weight(b).unwrap().clone();
        // let res = x + y;
        // let op = Addition(x.clone(), y.clone());
        // let grad = Op::Binary(x, y, BinaryOp::Add);
        let node = Node::Op {
            name: "add".to_string(),
            args: vec![a, b],
        };
        let c = self.graph.add_node(node);

        let _ac = self.graph.add_edge(a, c, None)?;
        let _bc = self.graph.add_edge(b, c, None)?;

        Ok(c)
    }
}

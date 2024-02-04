/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Config, GradientUpdater};
use crate::{ops::Evaluate, prelude::{Arithmetic, Constant, Variable, Op}};
use daggy::{Dag, NodeIndex};
use std::collections::HashMap;



pub enum Node<C: Config> {
    Var(Variable<<C as Config>::DType>),
}




pub struct FnGraph<C: Config> {
    graph: Dag<Node<C>, usize>,
    gradients: HashMap<usize, Option<GradientUpdater<C>>>,
}

impl<C> FnGraph<C> where C: Config {
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

    pub fn get(&self, index: NodeIndex) -> Option<&Node<C>> {
        self.graph.node_weight(index)
    }

    pub fn variable(&mut self, name: impl ToString, value: Option<<C as Config>::DType>) -> NodeIndex {
        let var = Variable::new(name, value);
        self.graph.add_node(Node::Var(var))
    }
}

// impl<C> Arithmetic<NodeIndex> for FnGraph<C> where C: Config, <C as Config>::DType: 'static {
//     fn add(&mut self, left: NodeIndex, right: NodeIndex) -> NodeIndex {
//         let a = self.graph.node_weight(left).unwrap().clone();
//         let b = self.graph.node_weight(right).unwrap().clone();
//         let c = self.graph.add_node(a + b);
//         self.graph.extend_with_edges(vec![(left, c), (right, c)]);
//         c
//     }

//     fn mul(&mut self, left: NodeIndex, right: NodeIndex) -> NodeIndex {
//         let op = Box::new(Mul);
//         let c = self.graph.add_node(Node::Op(op));
//         self.graph.extend_with_edges(vec![(left, c), (right, c)]);
//         c
//     }
// }

pub struct AddOp;

impl AddOp {
    pub fn new() -> Self {
        Self
    }
}

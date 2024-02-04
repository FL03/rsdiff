/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Config, GradientUpdater};
use crate::ops::Op;
use crate::prelude::{Arithmetic, Constant, GradientStore, Result, Store, Variable};
use daggy::petgraph::algo::toposort;
use daggy::{Dag, EdgeIndex, NodeIndex};
use num::traits::NumOps;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub enum Node<C: Config> {
    Constant(Constant<<C as Config>::DType>),
    Var(Variable<<C as Config>::DType>),
    Operand(Op<<C as Config>::DType>),
}

pub struct FnGraph<C: Config> {
    graph: Dag<Variable<<C as Config>::DType>, Option<GradientUpdater<C>>>,
    gradients: Arc<Mutex<HashMap<usize, Option<GradientUpdater<C>>>>>,
}

impl<C> FnGraph<C>
where
    C: Config,
{
    pub fn new() -> Self {
        Self {
            graph: Dag::new(),
            gradients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn clear(&mut self) {
        self.graph.clear();
        self.gradients.lock().unwrap().clear();
    }

    pub fn get(&self, index: NodeIndex) -> Option<&Variable<<C as Config>::DType>> {
        self.graph.node_weight(index)
    }

    pub fn variable(
        &mut self,
        name: impl ToString,
        value: Option<<C as Config>::DType>,
    ) -> NodeIndex {
        let var = Variable::new(name, value);
        self.graph.add_node(var)
    }
}

impl<C> FnGraph<C>
where
    C: Config,
    C::DType: Clone + Default + 'static,
{
    pub fn compute_gradients(&mut self, target: NodeIndex) -> Result<()> {
        let nodes = toposort(&self.graph, None)?;

        let mut gradients = GradientStore::new();
        gradients.insert(target, self.get(target).unwrap().clone());
        Ok(())
    }
}

impl<C> Arithmetic<NodeIndex> for FnGraph<C>
where
    C: Config<Store = GradientStore<usize>>,
    C::DType: Clone + Default + NumOps + 'static,
    C::Eval: Arithmetic<NodeIndex>,
{
    fn add(&mut self, a: NodeIndex, b: NodeIndex) -> Result<NodeIndex> {
        let x = self.graph.node_weight(a).unwrap().clone();
        let y = self.graph.node_weight(b).unwrap().clone();
        let res = x + y;
        let c = self.graph.add_node(res);
        let ex = self.graph.add_edge(a, c, None).unwrap();
        let ey = self.graph.add_edge(b, c, None)?;
        // let fg = Arc::new(move | gradient: &mut <C as Config>::Eval, store: &mut <C as Config>::Store, rhs | -> Result<()> {
        //     let ai = a.index();
        //     let bi = b.index();
        //     //
        //     if let Some(grad) = store.get(&ai) {
        //         let grad = gradient.add(*grad, b)?;
        //         store.add_gradient(self, ex.index(), &grad);
        //     }
        //     if let Some(grad) = store.get(&bi) {
        //         let grad = gradient.add(*grad, a)?;
        //         store.add_gradient(self, ey.index(), &grad);
        //     }
        //     Ok(())
        // });

        // self.gradients.lock().unwrap().insert(ex.index(), Some(fg));

        Ok(c)
    }

    fn mul(&mut self, a: NodeIndex, b: NodeIndex) -> Result<NodeIndex> {
        let x = self.graph.node_weight(a).unwrap().clone();
        let y = self.graph.node_weight(b).unwrap().clone();
        let res = x * y;
        let c = self.graph.add_node(res);
        self.graph
            .extend_with_edges([(a, c), (b, c)])
            .expect("Failed to add edge");
        Ok(c)
    }
}

pub struct AddOp;

impl AddOp {
    pub fn new() -> Self {
        Self
    }
}

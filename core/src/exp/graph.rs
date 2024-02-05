/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Addition, Config, GradientUpdater};
use crate::ops::{BinaryOp, Op};
use crate::prelude::{Arithmetic, Evaluate, GradientStore, Result, Store, Variable};
use daggy::petgraph::algo::toposort;
use daggy::{Dag, NodeIndex};
use num::traits::NumOps;
use std::collections::HashMap;
use std::sync::Arc;

pub struct FnGraph<C: Config> {
    graph: Dag<Variable<C::DType>, Option<Op<Variable<C::DType>>>>,
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
        let op = Addition(x.clone(), y.clone());
        let grad = Op::Binary(x, y, BinaryOp::Add);
        let c = self.graph.add_node(Variable::new("add", Some(op.eval())));
        self.graph.extend_with_edges([(a, c, Some(grad.clone())), (b, c, Some(grad))])?;
        Ok(c)
    }
}

impl<C> Arithmetic<NodeIndex> for FnGraph<C>
where
    C: Config<Store = GradientStore<usize>>,
    C::DType: Clone + Default + Evaluate<Output = Variable<C::DType>> + NumOps + 'static,
    C::Eval: Arithmetic<NodeIndex>,
{
    fn add(&mut self, a: NodeIndex, b: NodeIndex) -> Result<NodeIndex> {
        let x = self.graph.node_weight(a).unwrap().clone();
        let y = self.graph.node_weight(b).unwrap().clone();
        // let res = x + y;
        let op = Addition(x, y);
        let grad = Arc::new(op.clone());
        let c = self.graph.add_node(Variable::new("add", Some(op.eval())));
        // let ex = self.graph.add_edge(a, c, Some(grad.clone())).unwrap();
        // let ey = self.graph.add_edge(b, c, None)?;
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

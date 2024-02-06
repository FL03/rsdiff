/*
    Appellation: graphs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Graphs
//!
//!
pub use self::{edge::*, graph::*, node::*};

pub(crate) mod edge;
pub(crate) mod graph;
pub(crate) mod node;

pub mod basic;

pub use daggy::NodeIndex as Id;

use crate::prelude::{Evaluate, Gradient, GradientStore, Result, Variable};
use daggy::NodeIndex;
use std::marker::PhantomData;
use std::sync::Arc;

pub type GradientUpdater<C> = Arc<
    dyn Fn(&mut <C as Config>::Grad, &mut <C as Config>::Store, NodeIndex) -> Result<()>
        + Send
        + Sync,
>;

pub trait Config: Default {
    type DType;
    type Eval: Clone + Default;
    type Grad;
    type Store;
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Addition<T>(T, T);

impl<T> Addition<T> {
    pub fn new(a: T, b: T) -> Self {
        Self(a, b)
    }
}

impl<S, T> Evaluate for Addition<S>
where
    S: Evaluate<Output = T>,
    T: std::ops::Add<Output = T>,
{
    type Output = T;

    fn eval(self) -> Self::Output {
        self.0.eval() + self.1.eval()
    }
}

impl<T> Gradient<T> for Addition<T>
where
    T: Clone + Gradient<T> + std::ops::Add<Output = T>,
{
    type Gradient = Addition<<T as Gradient<T>>::Gradient>;

    fn grad(&self, args: T) -> Self::Gradient {
        Addition(self.0.grad(args.clone()), self.1.grad(args))
    }
}

// pub struct Backend

pub struct ComputeConfig<T, E> {
    dtype: PhantomData<T>,
    eval: PhantomData<E>,
}

impl<T, E> Default for ComputeConfig<T, E> {
    fn default() -> Self {
        Self {
            dtype: PhantomData,
            eval: PhantomData,
        }
    }
}

impl<T, E> Config for ComputeConfig<T, E>
where
    E: Clone + Default,
{
    type DType = T;
    type Eval = E;
    type Grad = E;
    type Store = GradientStore<usize>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmp::{Constant, Variable};

    #[test]
    fn test_addition() {
        // let mut dag = FnGraph::new();
        // let a = dag.variable('x', Some(1.0));
        // let b = dag.variable('y', Some(1.0));
        let add = Addition::new(Constant::new(1.0), Constant::new(1.0));
        assert_eq!(add.eval(), 2.0);
        let x = Variable::new("x", Some(1.0));
        let y = Variable::new("y", Some(1.0));
        let add = Addition::new(x.clone(), y.clone());
        assert_eq!(add.clone().eval(), 2.0);
        assert_eq!(add.grad(x.clone()).eval(), 1.0);
        assert_eq!(add.grad(x).eval(), add.grad(y).eval())
    }
}

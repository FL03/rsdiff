/*
    Appellation: graphs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Graphs
//!
//!
pub use self::{graph::*, node::*, edge::*};

pub(crate) mod graph;
pub(crate) mod node;
pub(crate) mod edge;

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

    #[test]
    fn test_dag() {
        // let mut dag = FnGraph::new();
        // let a = dag.variable('x', Some(1.0));
        // let b = dag.variable('y', Some(1.0));

        // assert_eq!(*dag.get(e).unwrap(), 2.0);
    }
}

/*
    Appellation: graphs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Graphs
//!
//!
pub use self::{graph::*, node::*, value::*};

pub(crate) mod graph;
pub(crate) mod node;
pub(crate) mod value;

use crate::ops::Evaluate;
use crate::prelude::{Constant, Gradient, GradientStore, Result, Variable};
use daggy::{Dag, NodeIndex};
use std::marker::PhantomData;
use std::sync::Arc;

pub type FnDag<T> = Dag<Value<T>, usize>;

pub type GradientUpdater<C> = Arc<
    dyn Fn(&<C as Config>::Eval, &mut <C as Config>::Store, NodeIndex) -> Result<()> + Send + Sync,
>;


pub trait Config: Default {
    type DType;
    type Eval: Clone + Default;
    type Store;
}

pub trait Operator<T> {
    type Grad;
    type Output;

    fn eval(&mut self, args: Vec<T>) -> Self::Output;

    fn grad(&self, at: impl Operator<T>) -> Self::Grad;
}

pub trait Opper<T> {
    type Grad;
    type Output;

    fn eval(&self) -> Self::Output;

    fn grad(&self, at: impl Operator<T>) -> Self::Grad;
}



// impl<T> Operator<T> for Variable<T> where T: Clone + Default {
//     type Grad = T;
//     type Output = T;

//     fn eval(&self) -> Self::Output {
//         self.value().map(|v| v.clone()).unwrap_or(T::default())
//     }

//     fn grad(&self, _: impl Operator<T>) -> Self::Grad {
//         Variable::grad()
//     }
// }

pub struct Addition<T>(T, T);

impl<T> Evaluate<(T, T)> for Addition<T> where T: Copy + std::ops::Add<Output = T> {
    type Output = T;

    fn eval(&self, args: (T, T)) -> Self::Output {
        args.0 + args.1
    }
}

impl<T> Gradient<T> for Addition<T> where T: Copy + std::ops::Add<Output = T> {
    type Gradient = (T, T);

    fn grad(&self, args: T) -> Self::Gradient {
        (args, args)
    }
}

// impl<T> Operator<T> for Addition<T> where T: Copy + Operator<T, Grad = T> + std::ops::Add<Output = T>, {
//     type Grad = (T, T);
//     type Output = T;

//     fn eval(&self) -> T {
//         self.0 + self.1
//     }

//     fn grad(&self, args: Self::Output) -> Self::Grad {
//         let dx = self.0.grad(args);
//         let dy = self.1.grad(args);
//         (dx, dy)
//     }
// }

// impl<T> Operator for 



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

impl<T, E> Config for ComputeConfig<T, E> where E: Clone + Default {
    type DType = T;
    type Eval = E;
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

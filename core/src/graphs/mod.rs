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

use crate::errors::Result;
use crate::stores::GradientStore;
use daggy::{Dag, NodeIndex};
use std::sync::Arc;

pub type FnDag<T> = Dag<Value<T>, usize>;

pub type GradientUpdater<C> = Arc<
    dyn Fn(&mut <C as Config>::Grad, &mut <C as Config>::Store, NodeIndex) -> Result<()>
        + Send
        + Sync,
>;

pub trait Config {
    type Eval: Clone + Default;
    type Grad;
    type Store;
}

pub trait CoreGraph<T> {
    type Value;

    fn constant(&mut self, value: T) -> Self::Value;
    fn variable(&mut self, value: T) -> Self::Value;
}

pub trait Arithmetic<T> {
    fn add(&mut self, a: T, b: T) -> T;
    fn mul(&mut self, a: T, b: T) -> T;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dag() {
        let mut dag = Graph::new();
        let a = dag.variable(1_f64);
        let b = dag.variable(1_f64);
        let c = dag.mul(a, b);

        let d = dag
            .operator(vec![a, b], |v: Vec<f64>| v.iter().product())
            .unwrap();

        let e = dag.add(c, a);

        println!("{:?}", &dag.get(c));
        assert_eq!(dag.get(c), dag.get(d));
        assert_eq!(*dag.get(e).unwrap(), 2.0);
    }
}

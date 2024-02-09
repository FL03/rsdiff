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

pub mod dynamic;

use crate::prelude::Result;
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
    fn add(&mut self, a: T, b: T) -> Result<T>;
    fn mul(&mut self, a: T, b: T) -> Result<T>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "not implemented"]
    #[test]
    fn test_dag() {
        let mut dag = Graph::new();
        let x = dag.variable(1_f64);
        let y = dag.variable(2_f64);
        

        let c = dag.add(x, y).unwrap();

        let d = dag.mul(c, y).unwrap();

        assert_eq!(*dag.get_value(c).unwrap(), 3.0);
        assert_eq!(*dag.get_value(d).unwrap(), 6.0);

        let gc = dag.gradient_at(c).unwrap();
        // todo: fix this
        assert_eq!(gc[&x], 1.0);
        assert_eq!(gc[&y], 1.0);

        let gd = dag.gradient_at(d).unwrap();

        assert_eq!(gd[&x], 2.0);
        assert_eq!(gd[&y], 5.0);
    }
}

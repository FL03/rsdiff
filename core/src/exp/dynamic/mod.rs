/*
    Appellation: dynamic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Dynamic Compute Graph
//!
//!
//!    - A dynamic computational graph considers a DAG whose nodes represent data in the form of tensors.
//!    - Edges represent the operations applied to the data.
pub use self::{edge::*, graph::*, node::*};

pub(crate) mod edge;
pub(crate) mod graph;
pub(crate) mod node;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dcg() {
        let mut dag = Dcg::new();
        let a = dag.variable(1_f64);
        let b = dag.variable(1_f64);
        // let c = dag.mul(a, b).unwrap();

        // let e = dag.add(c, a).unwrap();

        assert_eq!(*dag.get(a).unwrap().value().unwrap(), 1.0);
        // assert_eq!(*dag.get(e).unwrap(), 2.0);
    }
}

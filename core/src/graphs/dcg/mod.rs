/*
    Appellation: dcg <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Dynamic Compute Graph
//!
//! A computational graph forms the backbone of automatic differentiation. Computational graphs are directed acyclic graphs (DAGs)
//! that represent any computation as a series of nodes and edges.
pub use self::graph::Dcg;

pub(crate) mod graph;

pub mod edge;
pub mod node;

pub(crate) type DynamicGraph<T> = petgraph::graph::DiGraph<node::Node<T>, edge::Edge>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dcg() {
        let mut dcg = Dcg::<f64>::new();
        let a = dcg.input(true, 2.0);
        let b = dcg.input(true, 3.0);
        let c = dcg.add(a, b);

        let grad = dcg.gradient(c).unwrap();
        assert_eq!(grad[&a], 1.0);

        let mut dcg = Dcg::<f64>::new();
        let a = dcg.input(true, 2.0);
        let b = dcg.input(true, 3.0);
        let c = dcg.mul(a, b);

        let grad = dcg.gradient(c).unwrap();
        assert_eq!(grad[&a], 3.0);
        assert_eq!(grad[&b], 2.0);
    }
}

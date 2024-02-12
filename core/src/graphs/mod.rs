/*
    Appellation: graphs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Graphs
//!
//! A computational graph forms the backbone of automatic differentiation. Computational graphs are directed acyclic graphs (DAGs)
//! that represent any computation as a series of nodes and edges.
//!
//! In a dynamic computational graph (DCG), the graph considers the nodes to be tensors and the edges to be operations.
//!
pub use self::{edge::*, graph::*, node::*};

pub(crate) mod edge;
pub(crate) mod graph;
pub(crate) mod node;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dag() {
        let mut dag = Graph::new();
        let x = dag.variable(1_f64);
        let y = dag.variable(2_f64);
        // f(x, y) = x + y
        let c = dag.add(x, y).unwrap();
        // verify the value of c to be the sum of x and y
        assert_eq!(*dag.get_value(c).unwrap(), 3.0);
        // f(x, y) = y * (x + y)
        let d = dag.mul(c, y).unwrap();
        // verify the value of d to be the product of c and y
        assert_eq!(*dag.get_value(d).unwrap(), 6.0);

        let gc = dag.gradient_at(c).unwrap();

        assert_eq!(gc[&x], 1.0);
        assert_eq!(gc[&y], 1.0);

        let gd = dag.backward().unwrap();

        assert_eq!(gd[&x], 2.0);
        assert_eq!(gd[&y], 5.0);
    }

    #[test]
    fn test_backward() {
        let mut dag = Graph::new();
        let x = dag.variable(1_f64);
        let y = dag.variable(2_f64);

        let c = dag.sub(x, y).unwrap();

        let d = dag.mul(c, y).unwrap();

        assert_eq!(*dag.get_value(c).unwrap(), -1.0);
        assert_eq!(*dag.get_value(d).unwrap(), -2.0);

        let gc = dag.gradient_at(c).unwrap();

        assert_eq!(gc[&x], 1.0);
        assert_eq!(gc[&y], -1.0);

        let gd = dag.backward().unwrap();

        assert_eq!(gd[&x], 2.0);
        assert_eq!(gd[&y], -3.0);
    }

    #[ignore = "Not yet implemented"]
    #[test]
    fn test_division() {
        let mut dag = Graph::new();
        let one = dag.constant(1_f64);
        let x = dag.variable(1_f64);
        let y = dag.variable(2_f64);

        let c = dag.add(x, y).unwrap();

        let d = dag.div(one, c).unwrap();

        assert_eq!(*dag.get_value(c).unwrap(), 3.0);
        assert_eq!(*dag.get_value(d).unwrap(), 1.0 / 3.0);

        let gc = dag.gradient_at(c).unwrap();

        assert_eq!(gc[&x], 1.0);
        assert_eq!(gc[&y], 1.0);

        let gd = dag.backward().unwrap();

        assert_eq!(gd[&x], -1.0);
        assert_eq!(gd[&y], -1.0);
    }
}

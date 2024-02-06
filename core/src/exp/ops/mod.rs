/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{addition::*, multiply::*};

pub(crate) mod addition;
pub(crate) mod multiply;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmp::{Constant, Variable};
    use crate::ops::{Evaluate, Gradient};

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

    #[test]
    fn test_multiply() {
        let mul = Multiply::new(Constant::new(2.0), Constant::new(2.0));
        assert_eq!(mul.eval(), 4.0);
        let x = Variable::new("x", Some(2.0));
        let y = Variable::new("y", Some(2.0));
        let mul = Multiply::new(x.clone(), y.clone());
        assert_eq!(mul.clone().eval(), 4.0);
        // assert_eq!(mul.grad(x.clone()).eval(), 2.0);
    }
}
/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{addition::*, multiply::*};

pub(crate) mod addition;
pub(crate) mod multiply;

pub trait Grad<T> {
    type Gradient;

    fn grad(&self, at: T) -> Self::Gradient;
}

#[cfg(test)]
mod tests {
    use super::{Addition, Multiply};
    use crate::cmp::{Constant, Variable};
    use crate::ops::{Evaluate, Gradient};

    #[test]
    fn test_addition() {
        let add = Addition::new(Constant::new(1.0), Constant::new(1.0));
        assert_eq!(add.eval(), 2.0);
        let x = Variable::new("x").with_value(1.0);
        let y = Variable::new("y").with_value(2.0);
        let add = Addition::new(x.clone(), y.clone());
        assert_eq!(add.clone().eval(), 3.0);
        assert_eq!(add.grad(x.clone()), 1.0);
        assert_eq!(add.grad(y.clone()), 1.0);
        assert_eq!(add.grad(x.clone()).eval(), add.grad(y.clone()).eval());
    }

    #[test]
    fn test_multiply() {
        let mul = Multiply::new(Constant::new(2.0), Constant::new(2.0));
        assert_eq!(mul.eval(), 4.0);
        let x = Variable::new("x").with_value(2.0);
        let y = Variable::new("y").with_value(2.0);
        let mul = Multiply::new(x.clone(), y.clone());
        assert_eq!(mul.clone().eval(), 4.0);
        assert_eq!(mul.grad(x.clone()).eval(), 2.0);
    }
}

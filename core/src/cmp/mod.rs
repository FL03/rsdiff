/*
    Appellation: cmp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Components
//!
//!
pub use self::{constants::*, operators::*, variables::*};

pub(crate) mod constants;
pub(crate) mod operators;
pub(crate) mod variables;

pub mod id;

use daggy::NodeIndex;

pub trait Var<T> {
    fn name(&self) -> &str;

    fn evaluate<F>(&self) -> fn(T) -> T {
        std::convert::identity
    }
}

pub trait NodeConfig {
    type Eval;
    type Grad;
}

#[derive(Clone, Debug, PartialEq)]
pub enum FnNode<T> {
    Const(Constant<T>),
    Var(Variable<T>),
    Binary { left: NodeIndex, right: NodeIndex },
}

impl<T> FnNode<T> {
    pub fn constant(value: T) -> Self {
        Self::Const(Constant::new(value))
    }

    pub fn variable(name: impl ToString) -> Self {
        Self::Var(Variable::symbolic(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_node_constant() {
        let node = FnNode::constant(3);
        assert_eq!(node, FnNode::Const(Constant(3)));

        let value = Constant(3);
        let add = value + 3;
        assert_eq!(add, Constant(6));
    }
}

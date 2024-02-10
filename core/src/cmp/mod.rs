/*
    Appellation: cmp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Components
//!
//!
pub use self::{constants::*, dual::*, operators::*, variables::*};

pub(crate) mod constants;
pub(crate) mod dual;
pub(crate) mod operators;
pub(crate) mod variables;

pub mod id;

use daggy::NodeIndex;

pub trait NodeConfig {
    type Eval;
    type Grad;
}

#[derive(Clone, Debug, PartialEq)]
pub enum FnNode<T> {
    Const(Constant<T>),
    Var(Variable<T>),
    Binary { left: NodeIndex, right: NodeIndex },
    Operator {},
}

impl<T> FnNode<T> {
    pub fn constant(value: T) -> Self {
        Self::Const(Constant::new(value))
    }

    pub fn variable(name: impl ToString) -> Self {
        Self::Var(Variable::symbolic(name))
    }
}

macro_rules! impl_op {
    ($name:ident, $bound:ident, $fn:ident, $val:tt, $e:expr) => {
        impl<T> $bound for $name<T>
        where
            T: $bound<Output = T>,
        {
            type Output = Self;

            fn $fn(self, rhs: $name<T>) -> Self::Output {
                $e(self.$val, rhs.$val)
            }
        }

        impl<T> $bound<T> for $name<T>
        where
            T: $bound<Output = T>,
        {
            type Output = Self;

            fn $fn(self, rhs: T) -> Self::Output {
                $e(self.$val, rhs)
            }
        }
    };
}

macro_rules! impl_dual_op {
    ($name:ident, $bound:ident, $fn:ident, $val:tt, $e:expr) => {
        impl<T> $bound for $name<T>
        where
            T: $bound<Output = T>,
        {
            type Output = Self;

            fn $fn(self, rhs: $name<T>) -> Self::Output {
                $e(self.$val, rhs.$val)
            }
        }

        impl<T> $bound<T> for $name<T>
        where
            T: $bound<Output = T>,
        {
            type Output = Self;

            fn $fn(self, rhs: T) -> Self::Output {
                $e(self.$val, rhs)
            }
        }
    };
}


use std::ops::{Add, Div, Mul, Rem, Sub};

impl_op!(Constant, Add, add, 0, |a, b| Constant::new(a + b));
impl_op!(Constant, Div, div, 0, |a, b| Constant::new(a / b));
impl_op!(Constant, Mul, mul, 0, |a, b| Constant::new(a * b));
impl_op!(Constant, Rem, rem, 0, |a, b| Constant::new(a % b));
impl_op!(Constant, Sub, sub, 0, |a, b| Constant::new(a - b));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant() {
        let a = Constant(3);
        let add = a + 3;
        assert_eq!(add, Constant(6));

        let a = Constant::new(3);
        let b = Constant::new(3);
        assert_eq!(a + b, Constant(6));
    }

    #[test]
    fn test_fn_node_constant() {
        let node = FnNode::constant(3);
        assert_eq!(node, FnNode::Const(Constant(3)));

        let value = Constant(3);
        let add = value + 3;
        assert_eq!(add, Constant(6));
    }
}

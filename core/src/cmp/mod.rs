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
    ($name:ident, $val:tt) => {
        impl<T> std::ops::Add for $name<T>
        where
            T: std::ops::Add<Output = T>,
        {
            type Output = Self;

            fn add(self, rhs: $name<T>) -> Self::Output {
                $name::new(self.$val + rhs.$val)
            }
        }

        impl<T> std::ops::Add<T> for $name<T>
        where
            T: std::ops::Add<Output = T>,
        {
            type Output = Self;

            fn add(self, rhs: T) -> Self::Output {
                $name::new(self.$val + rhs)
            }
        }
    };
}

macro_rules! impl_add {
    ($name:ident) => {
        impl<T> std::ops::Add for $name<T>
        where
            T: std::ops::Add<Output = T>,
        {
            type Output = Self;

            fn add(self, rhs: $name<T>) -> Self::Output {
                $name::new(self.0 + rhs.0)
            }
        }

        impl<T> std::ops::Add<T> for $name<T>
        where
            T: std::ops::Add<Output = T>,
        {
            type Output = Self;

            fn add(self, rhs: T) -> Self::Output {
                $name::new(self.0 + rhs)
            }
        }
    };
}

macro_rules! impl_div {
    ($name:ident) => {
        impl<T> std::ops::Div for $name<T>
        where
            T: std::ops::Div<Output = T>,
        {
            type Output = Self;

            fn div(self, rhs: $name<T>) -> Self::Output {
                $name::new(self.0 / rhs.0)
            }
        }

        impl<T> std::ops::Div<T> for $name<T>
        where
            T: std::ops::Div<Output = T>,
        {
            type Output = Self;

            fn div(self, rhs: T) -> Self::Output {
                $name::new(self.0 / rhs)
            }
        }
    };
}

macro_rules! impl_mul {
    ($name:ident) => {
        impl<T> std::ops::Mul for $name<T>
        where
            T: std::ops::Mul<Output = T>,
        {
            type Output = Self;

            fn mul(self, rhs: $name<T>) -> Self::Output {
                $name::new(self.0 * rhs.0)
            }
        }

        impl<T> std::ops::Mul<T> for $name<T>
        where
            T: std::ops::Mul<Output = T>,
        {
            type Output = Self;

            fn mul(self, rhs: T) -> Self::Output {
                $name::new(self.0 * rhs)
            }
        }
    };
}

macro_rules! impl_rem {
    ($name:ident) => {
        impl<T> std::ops::Rem for $name<T>
        where
            T: std::ops::Rem<Output = T>,
        {
            type Output = Self;

            fn rem(self, rhs: $name<T>) -> Self::Output {
                $name::new(self.0 % rhs.0)
            }
        }

        impl<T> std::ops::Rem<T> for $name<T>
        where
            T: std::ops::Rem<Output = T>,
        {
            type Output = Self;

            fn rem(self, rhs: T) -> Self::Output {
                $name::new(self.0 % rhs)
            }
        }
    };
}

macro_rules! impl_sub {
    ($name:ident) => {
        impl<T> std::ops::Sub for $name<T>
        where
            T: std::ops::Sub<Output = T>,
        {
            type Output = $name<T>;

            fn sub(self, rhs: $name<T>) -> Self::Output {
                $name::new(self.0 - rhs.0)
            }
        }

        impl<T> std::ops::Sub<T> for $name<T>
        where
            T: std::ops::Sub<Output = T>,
        {
            type Output = $name<T>;

            fn sub(self, rhs: T) -> Self::Output {
                $name::new(self.0 - rhs)
            }
        }
    };
}
impl_op!(Constant, 0);
// impl_add!(Constant);
impl_div!(Constant);
impl_mul!(Constant);
impl_rem!(Constant);
impl_sub!(Constant);

// impl_add!(Variable);
// impl_div!(Variable);
// impl_mul!(Variable);
// impl_rem!(Variable);
// impl_sub!(Variable);

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

/*
    Appellation: arithmetic <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::BinaryOperation;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

pub trait ArithmeticOp<S, T = Self>: Add<S, Output = T> {}

macro_rules! operator {
    ($op:ident) => {
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
        pub struct $op;

        impl $op {
            pub fn new() -> Self {
                Self
            }

            pub fn name(&self) -> String {
                stringify!($op).to_lowercase()
            }
        }
    };
}

macro_rules! impl_binary_op {
    ($op:ident, $bound:ident, $operator:tt) => {
        operator!($op);

        impl<A, B, C> BinaryOperation<A, B> for $op
        where
            A: $bound<B, Output = C>,
        {
            type Output = C;

            fn eval(&self, lhs: A, rhs: B) -> Self::Output {
                lhs $operator rhs
            }
        }
    };
    (expr $op:ident, $bound:ident, $exp:expr) => {
        operator!($op);

        impl<A, B, C> BinaryOperation<A, B> for $op
        where
            A: $bound<B, Output = C>,
        {
            type Output = C;

            fn eval(&self, lhs: A, rhs: B) -> Self::Output {
                $exp(lhs, rhs)
            }
        }
    };
}

impl_binary_op!(Addition, Add, +);

impl_binary_op!(Division, Div, /);

impl_binary_op!(Multiplication, Mul, *);

impl_binary_op!(Subtraction, Sub, -);

#[derive(Clone)]
pub enum Arithmetic {
    Add(Addition),
    Div(Division),
    Mul(Multiplication),
    Sub(Subtraction),
}

impl Arithmetic {
    pub fn new(op: Arithmetic) -> Self {
        op
    }

    pub fn add() -> Self {
        Self::Add(Addition::new())
    }

    pub fn div() -> Self {
        Self::Div(Division::new())
    }

    pub fn mul() -> Self {
        Self::Mul(Multiplication::new())
    }

    pub fn sub() -> Self {
        Self::Sub(Subtraction::new())
    }

    pub fn op<A, B, C>(&self) -> Box<dyn BinaryOperation<A, B, Output = C>>
    where
        A: Add<B, Output = C> + Div<B, Output = C> + Mul<B, Output = C> + Sub<B, Output = C>,
    {
        match self.clone() {
            Arithmetic::Add(op) => Box::new(op),
            Arithmetic::Div(op) => Box::new(op),
            Arithmetic::Mul(op) => Box::new(op),
            Arithmetic::Sub(op) => Box::new(op),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Arithmetic::Add(op) => op.name(),
            Arithmetic::Div(op) => op.name(),
            Arithmetic::Mul(op) => op.name(),
            Arithmetic::Sub(op) => op.name(),
        }
    }

    pub fn eval<A, B, C>(&self, lhs: A, rhs: B) -> C
    where
        A: Add<B, Output = C> + Div<B, Output = C> + Mul<B, Output = C> + Sub<B, Output = C>,
    {
        match self {
            Arithmetic::Add(op) => op.eval(lhs, rhs),
            Arithmetic::Div(op) => op.eval(lhs, rhs),
            Arithmetic::Mul(op) => op.eval(lhs, rhs),
            Arithmetic::Sub(op) => op.eval(lhs, rhs),
        }
    }
}

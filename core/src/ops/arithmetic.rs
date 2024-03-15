/*
    Appellation: arithmetic <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

pub trait Trig {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
}

macro_rules! impl_binary_op {
    ($op:ident, $bound:ident, $exp:expr) => {
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
        pub struct $op;

        impl $op {
            pub fn new() -> Self {
                Self
            }
        }

        impl<A, B, C> crate::ops::BinaryOperation<A, B> for $op
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

impl_binary_op!(Addition, Add, |lhs, rhs| lhs + rhs);

impl_binary_op!(Division, Div, |lhs, rhs| lhs / rhs);

impl_binary_op!(Multiplication, Mul, |lhs, rhs| lhs * rhs);

impl_binary_op!(Subtraction, Sub, |lhs, rhs| lhs - rhs);

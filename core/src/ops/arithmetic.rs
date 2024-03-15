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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Addition;

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Division;

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Multiplication;

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Subtraction;

macro_rules! impl_binary_op {
    ($op:ident, $bound:ident, $exp:expr) => {
        impl $op {
            pub fn new() -> Self {
                Self
            }
        }

        impl<T> crate::ops::BinaryOperation<T> for $op
        where
            T: $bound<T, Output = T>,
        {
            type Output = T;

            fn eval(&self, lhs: T, rhs: T) -> Self::Output {
                $exp(lhs, rhs)
            }
        }
    };
}

impl_binary_op!(Addition, Add, |lhs, rhs| lhs + rhs);

impl_binary_op!(Division, Div, |lhs, rhs| lhs / rhs);

impl_binary_op!(Multiplication, Mul, |lhs, rhs| lhs * rhs);

impl_binary_op!(Subtraction, Sub, |lhs, rhs| lhs - rhs);

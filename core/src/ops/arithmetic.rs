use super::BinaryOperation;
use std::ops::{Add, Div, Mul, Sub};

pub struct Addition;

pub struct Division;

pub struct Multiplication;

pub struct Subtraction;

macro_rules! impl_binary_op {
    ($op:ident, $bound:ident, $exp:expr) => {
        impl<T> crate::ops::BinaryOperation<T> for $op where T: $bound<T, Output = T> {
            type Output = T;

            fn eval(&self, lhs: T, rhs: T) -> Self::Output {
                $exp(lhs, rhs)
            }
        }

        impl $op {
            pub fn new() -> Self {
                Self
            }
        }

        impl Default for $op {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

impl_binary_op!(Addition, Add, |lhs, rhs| lhs + rhs);

impl_binary_op!(Division, Div, |lhs, rhs| lhs / rhs);

impl_binary_op!(Multiplication, Mul, |lhs, rhs| lhs * rhs);

impl_binary_op!(Subtraction, Sub, |lhs, rhs| lhs - rhs);
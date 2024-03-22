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

macro_rules! impl_const_op {
    ($name:ident, $bound:ident, $fn:ident, $e:expr) => {
        impl_op!($name, $bound, $fn, 0, |a, b| $name::new($e(a, b)));
    };
}

use std::ops::{Add, Div, Mul, Rem, Sub};

impl_const_op!(Constant, Add, add, |a, b| a + b);
impl_const_op!(Constant, Div, div, |a, b| a / b);
impl_const_op!(Constant, Mul, mul, |a, b| a * b);
impl_const_op!(Constant, Rem, rem, |a, b| a % b);
impl_const_op!(Constant, Sub, sub, |a, b| a - b);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant() {
        let a = Constant(3);
        let add = a + 3;
        assert_eq!(add, Constant(6));

        // let b = Constant(3_f64).ln();

        let a = Constant::new(3);
        let b = Constant::new(3);
        assert_eq!(a + b, Constant(6));
    }
}

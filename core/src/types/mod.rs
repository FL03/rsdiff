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

/// A boxed error type for use in the library.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
/// A boxed result type for use in the library.
pub type BoxResult<T = ()> = std::result::Result<T, BoxError>;

macro_rules! impl_op {
    ($name:ident, $bound:ident, $fn:ident, $val:tt, $e:expr) => {
        impl<T> std::ops::$bound for $name<T>
        where
            T: std::ops::$bound<Output = T>,
        {
            type Output = Self;

            fn $fn(self, rhs: $name<T>) -> Self::Output {
                $e(self.$val, rhs.$val)
            }
        }

        impl<T> std::ops::$bound<T> for $name<T>
        where
            T: std::ops::$bound<Output = T>,
        {
            type Output = Self;

            fn $fn(self, rhs: T) -> Self::Output {
                $e(self.$val, rhs)
            }
        }
    };
}

macro_rules! impl_const_op {
    ($bound:ident, $fn:ident, $e:expr) => {
        impl_op!(Constant, $bound, $fn, 0, |a, b| Constant::new($e(a, b)));
    };
}

impl_const_op!(Add, add, |a, b| a + b);
impl_const_op!(Div, div, |a, b| a / b);
impl_const_op!(Mul, mul, |a, b| a * b);
impl_const_op!(Rem, rem, |a, b| a % b);
impl_const_op!(Sub, sub, |a, b| a - b);

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

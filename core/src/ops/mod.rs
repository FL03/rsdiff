/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Operations
//!
//!

pub use self::{expr::*, kinds::*, ops::*};
pub use self::{kinds::prelude::*, traits::prelude::*};

pub(crate) mod expr;
pub(crate) mod ops;

pub(crate) mod kinds {

    pub mod binary;
    pub mod nary;
    pub mod ternary;
    pub mod unary;

    pub(crate) mod prelude {
        pub use super::binary::{
            Arithmetic, ArithmeticAssign, BinOp, BinaryArgs, BinaryOp, BinaryParams,
        };
        pub use super::nary::NaryOp;
        pub use super::ternary::{TernaryExpr, TernaryOp};
        pub use super::unary::UnaryOp;
    }
}

pub(crate) mod traits {
    pub use self::prelude::*;

    pub mod evaluate;
    pub mod operand;
    pub mod operator;
    pub mod params;

    pub(crate) mod prelude {
        pub use super::evaluate::*;
        pub use super::operand::*;
        pub use super::operator::*;
        pub use super::params::*;
    }
}

pub trait IntoOp<F>
where
    F: Operator,
{
    fn into_op(self) -> F;
}

impl<S, F> IntoOp<F> for S
where
    F: Operator,
    S: Into<F>,
{
    fn into_op(self) -> F {
        self.into()
    }
}

pub(crate) mod prelude {
    pub use super::IntoOp;

    pub use super::kinds::prelude::*;
    pub use super::ops::{Op, OpKind};
    pub use super::traits::prelude::*;
}

#[cfg(test)]
mod tests {
    use super::{Arithmetic, Evaluate, Params};

    #[test]
    fn test_args() {
        let args = ();
        let pattern = args.into_pattern();
        assert_eq!(pattern, args);
        let args = (10f64,);
        let pattern = args.into_pattern();
        assert_eq!(pattern, args);
        let args = (0f64, 0f32);
        let pattern = args.into_pattern();
        assert_eq!(pattern, args);
        let args = (0f64, 0f32, 0usize);
        let pattern = args.into_pattern();
        assert_eq!(pattern, args);
    }

    #[test]
    fn test_arith() {
        let op = Arithmetic::add();
        assert_eq!(op.name(), "add");
        let res = op.eval((1f64, 2f64));
        assert_eq!(res, 3f64);
    }
}

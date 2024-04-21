/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Operations
//!
//!
pub use self::binary::{Arithmetic, BinaryOp};
pub use self::ternary::{TernaryExpr, TernaryOp};
pub use self::unary::UnaryOp;
pub use self::{expr::*, kinds::*, operator::*};

pub(crate) mod expr;
pub(crate) mod kinds;
pub(crate) mod operator;

pub mod binary;
pub mod ternary;
pub mod unary;

pub trait IntoOp<F> where F: Operator {
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

    pub use super::binary::*;
    pub use super::kinds::{Op, OpKind};
    pub use super::ternary::*;
    pub use super::unary::*;
}

#[cfg(test)]
mod tests {
    use super::binary::Arithmetic;
    use super::{Evaluator, Params};



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

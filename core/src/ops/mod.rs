/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Operations
//!
//!
pub use self::{binary::BinaryOp, kinds::*, operator::*, unary::UnaryOp};

pub(crate) mod kinds;
pub(crate) mod operator;

pub mod binary;
pub mod unary;

pub trait IntoOp {
    fn into_op(self) -> Op;
}

impl<S> IntoOp for S
where
    S: Into<Op>,
{
    fn into_op(self) -> Op {
        self.into()
    }
}

pub(crate) mod prelude {
    pub use super::IntoOp;

    pub use super::binary::*;
    pub use super::kinds::Op;
    pub use super::unary::*;
}

#[cfg(test)]
mod tests {
    use super::binary::Arithmetic;
    use super::Evaluator;

    #[test]
    fn test_arith() {
        let op = Arithmetic::add();
        assert_eq!(op.name(), "add");
        let res = op.eval((1f64, 2f64));
        assert_eq!(res, 3f64);
    }
}

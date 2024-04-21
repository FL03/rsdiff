/*
    Appellation: expr <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{BinaryOp, UnaryOp};
use crate::id::IndexId;
use crate::prelude::AnyBox;
use strum::EnumIs;

#[derive(Clone, Debug, EnumIs, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Expr<K = usize, V = AnyBox> {
    Binary(BinaryExpr<K, V>),
    Unary(UnaryExpr<K, V>),
    Constant(V),
    Variable { id: IndexId<K>, value: V },
}

impl<K, V> Expr<K, V> {
    pub fn binary(lhs: Expr<K, V>, rhs: Expr<K, V>, op: BinaryOp) -> Self {
        Self::Binary(BinaryExpr::new(lhs, rhs, op))
    }

    pub fn constant(value: V) -> Self {
        Self::Constant(value)
    }

    pub fn unary(arg: Expr<K, V>, op: UnaryOp) -> Self {
        Self::Unary(UnaryExpr::new(arg, op))
    }

    pub fn variable(idx: K, value: V) -> Self {
        Self::Variable {
            id: IndexId::from_index(idx),
            value,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BinaryExpr<K = usize, V = AnyBox> {
    lhs: Box<Expr<K, V>>,
    op: BinaryOp,
    rhs: Box<Expr<K, V>>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UnaryExpr<K = usize, V = AnyBox> {
    op: UnaryOp,
    recv: Box<Expr<K, V>>,
}

mod expr_impl {
    use super::{BinaryExpr, Expr, UnaryExpr};
    use crate::ops::{BinaryOp, UnaryOp};

    impl<K, V> BinaryExpr<K, V> {
        pub fn new(lhs: Expr<K, V>, rhs: Expr<K, V>, op: BinaryOp) -> Self {
            Self {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        }

        pub fn lhs(&self) -> &Expr<K, V> {
            &self.lhs
        }

        pub fn lhs_mut(&mut self) -> &mut Expr<K, V> {
            &mut self.lhs
        }

        pub fn op(&self) -> BinaryOp {
            self.op
        }

        pub fn op_mut(&mut self) -> &mut BinaryOp {
            &mut self.op
        }

        pub fn rhs(&self) -> &Expr<K, V> {
            &self.rhs
        }

        pub fn rhs_mut(&mut self) -> &mut Expr<K, V> {
            &mut self.rhs
        }
    }

    impl<K, V> UnaryExpr<K, V> {
        pub fn new(recv: Expr<K, V>, op: UnaryOp) -> Self {
            Self {
                recv: Box::new(recv),
                op,
            }
        }

        pub fn op(&self) -> UnaryOp {
            self.op
        }

        pub fn op_mut(&mut self) -> &mut UnaryOp {
            &mut self.op
        }

        pub fn recv(&self) -> &Expr<K, V> {
            &self.recv
        }

        pub fn recv_mut(&mut self) -> &mut Expr<K, V> {
            &mut self.recv
        }
    }
}

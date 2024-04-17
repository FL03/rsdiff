/*
    Appellation: expr <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{BinaryOp, Op, OpKind, Operator, Params, UnaryOp};
use crate::prelude::AnyBox;
use strum::EnumIs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BinaryExpr<K = usize, V = AnyBox> {
    lhs: Box<Exprs<K, V>>,
    op: BinaryOp,
    rhs: Box<Exprs<K, V>>,
}

impl<K, V> BinaryExpr<K, V> {
    pub fn new(lhs: Exprs<K, V>, rhs: Exprs<K, V>, op: BinaryOp) -> Self {
        Self {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }

    pub fn lhs(&self) -> &Exprs<K, V> {
        &self.lhs
    }

    pub fn lhs_mut(&mut self) -> &mut Exprs<K, V> {
        &mut self.lhs
    }

    pub fn op(&self) -> BinaryOp {
        self.op
    }

    pub fn op_mut(&mut self) -> &mut BinaryOp {
        &mut self.op
    }

    pub fn rhs(&self) -> &Exprs<K, V> {
        &self.rhs
    }

    pub fn rhs_mut(&mut self) -> &mut Exprs<K, V> {
        &mut self.rhs
    }
}
#[derive(Clone, Debug, EnumIs, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Exprs<K = usize, V = AnyBox> {
    Binary(BinaryExpr<K, V>),
    Unary { arg: Box<Exprs<K, V>>, op: UnaryOp },
    Constant(V),
    Variable { id: K, value: V },
}

impl<K, V> Exprs<K, V> {
    pub fn binary(lhs: Exprs<K, V>, rhs: Exprs<K, V>, op: BinaryOp) -> Self {
        Self::Binary(BinaryExpr::new(lhs, rhs, op))
    }

    pub fn constant(value: V) -> Self {
        Self::Constant(value)
    }

    pub fn unary(arg: Exprs<K, V>, op: UnaryOp) -> Self {
        Self::Unary {
            arg: Box::new(arg),
            op,
        }
    }

    pub fn variable(id: K, value: V) -> Self {
        Self::Variable { id, value }
    }
}
pub struct Expr<Args>
where
    Args: Params,
{
    args: Args,
    op: Op,
}

impl<Args> Expr<Args>
where
    Args: Params,
{
    pub fn new(args: Args, op: Op) -> Self {
        Self { args, op }
    }

    pub fn args(&self) -> &Args {
        &self.args
    }

    pub fn args_mut(&mut self) -> &mut Args {
        &mut self.args
    }

    pub fn op(&self) -> Op {
        self.op
    }

    pub fn op_mut(&mut self) -> &mut Op {
        &mut self.op
    }
}

impl<Args: Params> Operator for Expr<Args> {
    fn name(&self) -> &str {
        self.op.name()
    }

    fn kind(&self) -> OpKind {
        self.op().kind()
    }
}

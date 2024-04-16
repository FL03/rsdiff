/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{BinaryOp, Operator, Params, UnaryOp};
use strum::{Display, EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    EnumCount,
    EnumDiscriminants,
    EnumIs,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize,),
    serde(rename_all = "lowercase", untagged),
    strum_discriminants(derive(serde::Deserialize, serde::Serialize))
)]
#[strum(serialize_all = "lowercase")]
#[strum_discriminants(
    derive(
        Display,
        EnumCount,
        EnumIs,
        EnumIter,
        EnumString,
        Hash,
        Ord,
        PartialOrd,
        VariantNames
    ),
    name(OpKind)
)]
pub enum Op {
    Binary(BinaryOp),
    Unary(UnaryOp),
}

impl Operator for Op {
    fn name(&self) -> &str {
        match self {
            Self::Binary(op) => op.name(),
            Self::Unary(op) => op.name(),
        }
    }

    fn kind(&self) -> OpKind {
        match self {
            Self::Binary(op) => op.kind(),
            Self::Unary(op) => op.kind(),
        }
    }
}

impl From<BinaryOp> for Op {
    fn from(op: BinaryOp) -> Self {
        Self::Binary(op)
    }
}

impl From<UnaryOp> for Op {
    fn from(op: UnaryOp) -> Self {
        Self::Unary(op)
    }
}

pub trait Expression<Args>: Sized
where
    Args: Params,
{
    fn args(&self) -> &Args;

    fn args_mut(&mut self) -> &mut Args;

    fn op(&self) -> Op;
}
pub enum Exprs<Id = usize> {
    Binary {
        lhs: Box<Exprs<Id>>,
        rhs: Box<Exprs<Id>>,
        op: BinaryOp,
    },
    Unary {
        arg: Box<Exprs<Id>>,
        op: UnaryOp,
    },
    Constant(Box<dyn core::any::Any>),
    Variable {
        id: Id,
        value: Box<dyn core::any::Any>,
    },
}

impl<Id> Exprs<Id> {
    pub fn binary(lhs: Exprs<Id>, rhs: Exprs<Id>, op: BinaryOp) -> Self {
        Self::Binary {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            op,
        }
    }

    pub fn unary(arg: Exprs<Id>, op: UnaryOp) -> Self {
        Self::Unary {
            arg: Box::new(arg),
            op,
        }
    }

    pub fn variable<T: 'static>(id: Id, value: T) -> Self {
        Self::Variable {
            id,
            value: Box::new(value),
        }
    }

    pub fn lhs(&self) -> Option<&Exprs<Id>> {
        match self {
            Self::Binary { lhs, .. } => Some(lhs),
            _ => None,
        }
    }

    pub fn lhs_mut(&mut self) -> Option<&mut Exprs<Id>> {
        match self {
            Self::Binary { lhs, .. } => Some(lhs),
            _ => None,
        }
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

impl<Args> Expression<Args> for Expr<Args>
where
    Args: Params,
{
    fn args(&self) -> &Args {
        &self.args
    }

    fn args_mut(&mut self) -> &mut Args {
        &mut self.args
    }

    fn op(&self) -> Op {
        self.op
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

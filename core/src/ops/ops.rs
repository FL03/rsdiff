/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::traits::OperandType;
use crate::ops::{BinaryOp, NaryOp, Operator, TernaryOp, UnaryOp};
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
    Nary(NaryOp),
    Ternary(TernaryOp),
    Unary(UnaryOp),
}

impl OpKind {
    pub fn from_type(op: impl OperandType) -> Self {
        op.kind()
    }

    pub fn optype(&self) -> Box<dyn OperandType> {
        use crate::ops::{Binary, Nary, Ternary, Unary};
        match self {
            OpKind::Binary => Box::new(Binary),
            OpKind::Nary => Box::new(Nary),
            OpKind::Ternary => Box::new(Ternary),
            OpKind::Unary => Box::new(Unary),
        }
    }
}
impl Op {
    pub fn binary(op: BinaryOp) -> Self {
        Self::Binary(op)
    }

    pub fn nary(op: NaryOp) -> Self {
        Self::Nary(op)
    }

    pub fn ternary(op: TernaryOp) -> Self {
        Self::Ternary(op)
    }

    pub fn unary(op: UnaryOp) -> Self {
        Self::Unary(op)
    }

    pub fn operator(&self) -> Box<dyn Operator> {
        match self.clone() {
            Self::Binary(op) => Box::new(op),
            Self::Nary(op) => Box::new(op),
            Self::Ternary(op) => Box::new(op),
            Self::Unary(op) => Box::new(op),
        }
    }
}

impl Operator for Op {
    fn name(&self) -> &str {
        match self {
            Self::Binary(op) => op.name(),
            Self::Nary(op) => op.name(),
            Self::Ternary(op) => op.name(),
            Self::Unary(op) => op.name(),
        }
    }

    fn kind(&self) -> OpKind {
        self.operator().kind()
    }
}

macro_rules! impl_from_op {
    ($($var:ident($op:ident)),*) => {
        $(
            impl_from_op!(@impl $var($op));
        )*
    };
    (@impl $var:ident($op:ident)) => {
        impl From<$op> for Op {
            fn from(op: $op) -> Self {
                Self::$var(op)
            }
        }
    };

}

impl_from_op! {
    Binary(BinaryOp),
    Nary(NaryOp),
    Ternary(TernaryOp),
    Unary(UnaryOp)
}

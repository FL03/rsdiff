/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::{OpKind, Operator};
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
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
    strum(serialize_all = "lowercase")
)]
#[repr(u8)]
pub enum UnaryOp {
    Abs,
    Cos,
    Cosh,
    Exp,
    Floor,
    #[cfg_attr(feature = "serde", serde(alias = "inverse"))]
    Inv,
    Ln,
    Neg,
    Not,
    #[cfg_attr(feature = "serde", serde(alias = "reciprocal"))]
    Recip,
    Sin,
    Sinh,
    #[cfg_attr(feature = "serde", serde(alias = "square_root"))]
    Sqrt,
    Square,
    Tan,
    Tanh,
}

impl UnaryOp {
    pub fn differentiable(&self) -> bool {
        match self {
            UnaryOp::Floor | UnaryOp::Inv => false,
            _ => true,
        }
    }

    variant_constructor!(
        (Abs, abs),
        (Cos, cos),
        (Cosh, cosh),
        (Exp, exp),
        (Floor, floor),
        (Inv, inv),
        (Ln, ln),
        (Neg, neg),
        (Not, not),
        (Recip, recip),
        (Sin, sin),
        (Sinh, sinh),
        (Sqrt, sqrt),
        (Square, square),
        (Tan, tan),
        (Tanh, tanh)
    );
}

impl Operator for UnaryOp {
    fn name(&self) -> &str {
        match self {
            UnaryOp::Abs => "abs",
            UnaryOp::Cos => "cos",
            UnaryOp::Cosh => "cosh",
            UnaryOp::Exp => "exp",
            UnaryOp::Floor => "floor",
            UnaryOp::Inv => "inv",
            UnaryOp::Ln => "ln",
            UnaryOp::Neg => "neg",
            UnaryOp::Not => "not",
            UnaryOp::Recip => "recip",
            UnaryOp::Sin => "sin",
            UnaryOp::Sinh => "sinh",
            UnaryOp::Sqrt => "sqrt",
            UnaryOp::Square => "square",
            UnaryOp::Tan => "tan",
            UnaryOp::Tanh => "tanh",
        }
    }

    fn kind(&self) -> OpKind {
        OpKind::Unary
    }
}

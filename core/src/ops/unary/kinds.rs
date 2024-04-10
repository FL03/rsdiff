/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize,),
    serde(rename_all = "lowercase", untagged)
)]
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
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum UnaryOp {
    Abs,
    Cos,
    Cosh,
    Exp,
    Floor,
    #[cfg_attr(
        feature = "serde",
        serde(alias = "inverse", alias = "recip", alias = "reciprocal")
    )]
    Inv,
    Ln,
    Neg,
    Not,
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
        (Sin, sin),
        (Sinh, sinh),
        (Sqrt, sqrt),
        (Square, square),
        (Tan, tan),
        (Tanh, tanh)
    );
}

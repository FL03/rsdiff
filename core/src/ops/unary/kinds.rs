/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
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
    #[cfg_attr(
        feature = "serde",
        serde(alias = "inverse")
    )]
    Inv,
    Ln,
    Neg,
    Not,
    #[cfg_attr(
        feature = "serde",
        serde(alias = "reciprocal")
    )]
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

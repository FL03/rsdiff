/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

#[derive(Clone)]
pub enum Op<T> {
    Binary(T, T, BinaryOp),
    Compare(T, T, CompareOp),
    Custom(String),
    Unary(T, UnaryOp),
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    VariantNames,
)]
#[repr(u8)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CompareOp {
    #[default]
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
    Ne,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
    VariantNames,
)]
#[repr(u8)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum BinaryOp {
    #[default]
    Add,
    Div,
    Maximum,
    Minimum,
    Mul,
    Sub,
}

impl BinaryOp {
    pub fn is_commutative(&self) -> bool {
        match self {
            Self::Add | Self::Mul => true,
            _ => false,
        }
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
    VariantNames,
)]
#[repr(u8)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum UnaryOp {
    #[default]
    Abs,
    Ceil,
    Cos,
    Cosh,
    Exp,
    Inverse, // or Reciprocal
    Floor,
    Log,
    Neg,
    Round,
    Rsqrt,
    Sin,
    Sinh,
    Sqrt,
    Tan,
    Tanh,
}

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
    VariantNames,
)]
#[repr(u8)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Ops {
    Binary(BinaryOp),
    Compare(CompareOp),
    #[default]
    Unary(UnaryOp),
    Custom {
        name: String,
    },
}

impl Ops {
    /// A functional constructor for [Ops::Binary]
    pub fn binary(op: BinaryOp) -> Self {
        Self::Binary(op)
    }
    /// A functional constructor for [Ops::Compare]
    pub fn compare(op: CompareOp) -> Self {
        Self::Compare(op)
    }
    /// A functional constructor for [Ops::Custom]
    pub fn custom(name: impl Into<String>) -> Self {
        Self::Custom {
            name: name.into(),
        }
    }
    /// A functional constructor for [Ops::Unary]
    pub fn unary(op: UnaryOp) -> Self {
        Self::Unary(op)
    }
}

impl From<BinaryOp> for Ops {
    fn from(op: BinaryOp) -> Self {
        Self::Binary(op)
    }
}

impl From<CompareOp> for Ops {
    fn from(op: CompareOp) -> Self {
        Self::Compare(op)
    }
}

impl From<UnaryOp> for Ops {
    fn from(op: UnaryOp) -> Self {
        Self::Unary(op)
    }
}

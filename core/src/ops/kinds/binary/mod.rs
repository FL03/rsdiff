/*
   Appellation: binary <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{args::*, arithmetic::*, specs::*};

pub(crate) mod args;
pub(crate) mod arithmetic;
pub(crate) mod specs;

use crate::ops::{Binary, Evaluate, Operand};
use smart_default::SmartDefault;
use strum::{AsRefStr, Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

impl<O, P, A, B> Evaluate<P> for O
where
    O: BinOp<A, B> + Operand<Kind = Binary>,
    P: BinaryParams<Lhs = A, Rhs = B>,
{
    type Output = <O as BinOp<A, B>>::Output;

    fn eval(&self, args: P) -> Self::Output {
        let (lhs, rhs) = args.into_pattern();
        BinOp::eval(self, lhs, rhs)
    }
}

#[derive(
    AsRefStr,
    Clone,
    Copy,
    Debug,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    EnumString,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    SmartDefault,
    VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase", untagged)
)]
#[non_exhaustive]
#[repr(C)]
#[strum(serialize_all = "lowercase")]
pub enum BinaryOp {
    // <Kind = String> {
    #[default]
    Arith(Arithmetic),
    ArithAssign(ArithmeticAssign),
    Max,
    Min,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    // Custom()
}

impl BinaryOp {
    pub fn differentiable(&self) -> bool {
        match self {
            Self::Arith(_) | Self::ArithAssign(_) => true,
            _ => false,
        }
    }

    pub fn is_commutative(&self) -> bool {
        match self {
            Self::Arith(arith) => arith.is_commutative(),
            BinaryOp::And | BinaryOp::Or | BinaryOp::Xor => true,
            _ => false,
        }
    }
    nested_constructor!(
        Arith<Arithmetic>,
        arithmetic,
        [add, div, mul, pow, rem, sub]
    );

    nested_constructor!(
        ArithAssign<ArithmeticAssign>,
        arithmetic_assign,
        [add_assign, div_assign, mul_assign, rem_assign, sub_assign]
    );

    // simple_enum_constructor!(
    //     st Custom, custom, { id: usize }
    // );
    variant_constructor!(
        (Max, max),
        (Min, min),
        (And, bitand),
        (Or, bitor),
        (Xor, bitxor),
        (Shl, shl),
        (Shr, shr)
    );
}

impl Operand for BinaryOp {
    type Kind = Binary;

    fn name(&self) -> &str {
        match self {
            Self::Arith(inner) => inner.name(),
            Self::ArithAssign(inner) => inner.name(),
            Self::Max => "max",
            Self::Min => "min",
            Self::And => "and",
            Self::Or => "or",
            Self::Xor => "xor",
            Self::Shl => "shl",
            Self::Shr => "shr",
        }
    }

    fn optype(&self) -> Self::Kind {
        Binary
    }
}

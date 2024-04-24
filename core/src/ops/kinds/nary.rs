/*
   Appellation: nary <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::{Evaluate, Nary, OpKind, Operand, Params};
use strum::{AsRefStr, Display, EnumCount, EnumIs, EnumIter, VariantNames};

#[derive(
    AsRefStr,
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
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase", untagged)
)]
#[strum(serialize_all = "lowercase")]
pub enum NaryOp {
    Product(Product),
    Sum(Sum),
}

impl NaryOp {
    pub fn sum() -> Self {
        Self::Sum(Sum::new())
    }

    pub fn name(&self) -> &str {
        self.as_ref()
    }

    pub fn kind(&self) -> OpKind {
        OpKind::Nary
    }
}

impl Operand for NaryOp {
    type Kind = Nary;

    fn name(&self) -> &str {
        self.as_ref()
    }

    fn optype(&self) -> Self::Kind {
        Nary
    }
}

operation!(Product<Nary>.product, Sum<Nary>.sum);

impl<P, T> Evaluate<P> for Product
where
    T: Clone + core::iter::Product,
    P: Params<Pattern = Vec<T>>,
{
    type Output = T;

    fn eval(&self, params: P) -> T {
        params.into_pattern().iter().cloned().product()
    }
}

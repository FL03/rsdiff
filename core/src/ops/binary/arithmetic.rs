/*
    Appellation: arithmetic <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{BinOp, BoxedBinOp};
use crate::ops::{Operator, OperatorKind};
use num::traits::NumOps;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

macro_rules! operator {
    ($kind:ident: $($op:ident),*) => {
        $(
            operator!($op, $kind);
        )*
    };
    ($op:ident, $kind:ident) => {

        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
        pub struct $op;

        impl $op {
            pub fn new() -> Self {
                Self
            }

            pub fn name(&self) -> &str {
                stringify!($op)
            }
        }

        impl core::fmt::Display for $op {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.name())
            }
        }

        impl Operator for $op {

            fn kind(&self) -> OperatorKind {
                OperatorKind::$kind
            }

            fn name(&self) -> &str {
                self.name()
            }
        }
    };

}

macro_rules! operators {
    ($group:ident: [$(($variant:ident, $op:ident, $method:ident)),*]) => {
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
            derive(Deserialize, Serialize,),
            serde(rename_all = "lowercase", untagged)
        )]
        #[repr(u8)]
        #[strum(serialize_all = "lowercase")]
        pub enum $group {
            $(
                $variant($op),
            )*
        }

        impl $group {
            $(
                pub fn $method() -> Self {
                    Self::$variant($op::new())
                }
            )*


            pub fn eval<A, B, C>(&self, lhs: A, rhs: B) -> C
            where
                A: NumOps<B, C>,
            {
                self.op().eval(lhs, rhs)
            }

            pub fn op<A, B, C>(self) -> BoxedBinOp<A, B, C>
            where
                A: NumOps<B, C>,
            {
                match self {
                    $(
                        $group::$variant(op) => Box::new(op),
                    )*
                }
            }

            pub fn name(&self) -> &str {
                match self {
                    $(
                        $group::$variant(op) => op.name(),
                    )*
                }
            }
        }

        impl Operator for $group {
            fn kind(&self) -> OperatorKind {
                OperatorKind::Binary
            }

            fn name(&self) -> &str {
                self.name()
            }
        }
    };
}

macro_rules! impl_binary_op {
    ($(($op:ident, $bound:ident, $operator:tt)),*) => {
        $(
            impl_binary_op!($op, $bound, $operator);
        )*

    };
    ($op:ident, $bound:ident, $operator:tt) => {
        operator!($op, Binary);

        impl<A, B, C> BinOp<A, B> for $op
        where
            A: core::ops::$bound<B, Output = C>,
        {
            type Output = C;

            fn eval(&self, lhs: A, rhs: B) -> Self::Output {
                lhs $operator rhs
            }
        }
    };
    (other: $op:ident, $bound:ident, $call:ident) => {
        operator!($op, Binary);

        impl<A, B, C> BinOp<A, B> for $op
        where
            A: $bound<B, Output = C>,
        {
            type Output = C;

            fn eval(&self, lhs: A, rhs: B) -> Self::Output {
                $bound::$call(lhs, rhs)
            }
        }
    };
}

impl_binary_op!((Addition, Add, +), (Division, Div, /), (Multiplication, Mul, *), (Remainder, Rem, %), (Subtraction, Sub, -));

use num::traits::Pow;

impl_binary_op!(other: Power, Pow, pow);

operators!(
    Arithmetic: [
        (Add, Addition, add),
        (Div, Division, div),
        (Mul, Multiplication, mul),
        (Rem, Remainder, rem),
        (Sub, Subtraction, sub)
    ]
);

impl Arithmetic {
    pub fn new(op: Arithmetic) -> Self {
        op
    }
}

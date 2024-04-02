/*
    Appellation: arithmetic <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{BinaryOperation, Operator};
use num::traits::NumOps;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

macro_rules! operator {
    ($op:ident) => {

        #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
        pub struct $op;

        impl $op {
            pub fn new() -> Self {
                Self
            }

            pub fn name(&self) -> String {
                stringify!($op).to_lowercase()
            }
        }

        impl Operator for $op {
            fn name(&self) -> String {
                self.name()
            }
        }
    };
    ($($op:ident),*) => {
        $(
            operator!($op);
        )*
    };

}

macro_rules! operators {
    ($group:ident; {$($variant:ident: $op:ident => $method:ident),*}) => {
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

            pub fn name(&self) -> String {
                match self {
                    $(
                        $group::$variant(op) => op.name(),
                    )*
                }
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
        operator!($op);

        impl<A, B, C> BinaryOperation<A, B> for $op
        where
            A: core::ops::$bound<B, Output = C>,
        {
            type Output = C;

            fn eval(&self, lhs: A, rhs: B) -> Self::Output {
                lhs $operator rhs
            }
        }
    };
    (expr $op:ident, $bound:ident, $exp:expr) => {
        operator!($op);

        impl<A, B, C> BinaryOperation<A, B> for $op
        where
            A: core::ops::$bound<B, Output = C>,
        {
            type Output = C;

            fn eval(&self, lhs: A, rhs: B) -> Self::Output {
                $exp(lhs, rhs)
            }
        }
    };
}

operators!(Arithmetic; {Add: Addition => add, Div: Division => div, Mul: Multiplication => mul, Rem: Remainder => rem, Sub: Subtraction => sub});

impl_binary_op!((Addition, Add, +), (Division, Div, /), (Multiplication, Mul, *), (Remainder, Rem, %), (Subtraction, Sub, -));


impl Arithmetic {
    pub fn new(op: Arithmetic) -> Self {
        op
    }

    pub fn into_op<A, B, C>(self) -> Box<dyn BinaryOperation<A, B, Output = C>>
    where
        A: NumOps<B, C>,
    {
        match self {
            Arithmetic::Add(op) => Box::new(op),
            Arithmetic::Div(op) => Box::new(op),
            Arithmetic::Mul(op) => Box::new(op),
            Arithmetic::Rem(op) => Box::new(op),
            Arithmetic::Sub(op) => Box::new(op),
        }
    }

    pub fn op<A, B, C>(&self) -> Box<dyn BinaryOperation<A, B, Output = C>>
    where
        A: NumOps<B, C>,
    {
        self.into_op()
    }

    pub fn eval<A, B, C>(&self, lhs: A, rhs: B) -> C
    where
        A: NumOps<B, C>,
    {
        self.op().eval(lhs, rhs)
    }
}

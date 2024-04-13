/*
    Appellation: arithmetic <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::BinOp;
use crate::ops::{Evaluator, OpKind, Operator};
use num::traits::{NumOps, Pow};
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

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
            derive(serde::Deserialize, serde::Serialize,),
            serde(rename_all = "lowercase", untagged),
        )]
        #[repr(C)]
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
                A: NumOps<B, C> + Pow<B, Output = C>,
                Box<dyn Operator>: Evaluator<(A, B), Output = C>,


            {
                self.op().eval((lhs, rhs))
            }


            pub fn name(&self) -> &str {
                match self {
                    $(
                        $group::$variant(op) => op.name(),
                    )*
                }
            }


            pub fn op(self) -> Box<dyn Operator> {
                match self {
                    $(
                        $group::$variant(op) => Box::new(op),
                    )*
                }
            }


        }

        impl Operator for $group {
            fn kind(&self) -> OpKind {
                OpKind::Binary
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
    ($operator:ident -> $(($op:ident, $bound:ident)),*) => {
        $(
            impl_binary_op!(other: $op, $bound, $operator);
        )*

    };
    (std: $(($op:ident, $bound:ident, $operator:ident)),*) => {
        $(
            impl_binary_op!(@core $op, $bound, $operator);
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
    (@core $op:ident, $bound:ident, $call:ident) => {
        operator!($op, Binary);

        impl<A, B, C> BinOp<A, B> for $op
        where
            A: core::ops::$bound<B, Output = C>,
        {
            type Output = C;

            fn eval(&self, lhs: A, rhs: B) -> Self::Output {
                core::ops::$bound::$call(lhs, rhs)
            }
        }
    };
    (other: $operand:ident, $bound:ident, $op:ident) => {
        operator!($operand, Binary);

        impl_binary_op!(@call $operand, $bound, $op);
    };

    (@call $operand:ident, $bound:ident, $op:ident) => {
        impl<A, B, C> BinOp<A, B> for $operand
        where
            A: $bound<B, Output = C>,
        {
            type Output = C;

            fn eval(&self, lhs: A, rhs: B) -> Self::Output {
                $bound::$op(lhs, rhs)
            }
        }
    };
    (@sym $operand:ident, $bound:ident, $op:tt) => {
        impl<A, B, C> BinOp<A, B> for $operand
        where
            A: $bound<B, Output = C>,
        {
            type Output = C;

            fn eval(&self, lhs: A, rhs: B) -> Self::Output {
                lhs $op rhs
            }
        }
    };
}

impl_binary_op!(
    (Addition, Add, +),
    (Division, Div, /),
    (Multiplication, Mul, *),
    (Remainder, Rem, %),
    (Subtraction, Sub, -)
);

impl_binary_op!(other: Power, Pow, pow);

impl_binary_op!(std:
    (BitAnd, BitAnd, bitand),
    (BitOr, BitOr, bitor),
    (BitXor, BitXor, bitxor),
    (Shl, Shl, shl),
    (Shr, Shr, shr)
);

operators!(
    Arithmetic: [
        (Add, Addition, add),
        (Div, Division, div),
        (Mul, Multiplication, mul),
        (Pow, Power, pow),
        (Rem, Remainder, rem),
        (Sub, Subtraction, sub)
    ]
);

impl Arithmetic {
    pub fn new(op: Arithmetic) -> Self {
        op
    }
}

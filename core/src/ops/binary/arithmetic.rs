/*
    Appellation: arithmetic <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::BinOp;
use crate::ops::{Evaluator, OpKind, Operator, Params};
use num::traits::{NumOps, Pow};
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

macro_rules! impl_arith {
    ($parent:ident: {$($var:ident($inner:ident): $new:ident),*}) => {
        impl_arith!($parent: [$($var, $inner, $new),*]);
    };
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

            pub fn name(&self) -> &str {
                match self {
                    $(
                        $group::$variant(_) => stringify!($method),
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

        impl<P, A, B, C> Evaluator<P> for $group
        where
            A: NumOps<B, C> + Pow<B, Output = C>,
            P: Params<Pattern = (A, B)>,
        {
            type Output = C;

            fn eval(&self, args: P) -> Self::Output

            {
                match self {
                    $(
                        $group::$variant(op) => Evaluator::eval(op, args),
                    )*
                }
            }
        }
    };
}

macro_rules! impl_binary_op {
    // ($($args:tt),*) => {
    //     impl_binary_op!(@loop $($args),*);

    // };
    ($op:ident, $($p:ident)::*, $operator:tt) => {
        impl_binary_op!(@loop $op, $($p)::*, $operator);
    };
    (other: $operand:ident, $($p:ident)::*, $op:ident) => {
        impl_binary_op!(@loop $operand, $($p)::*, $op);
    };
    (std $(($op:ident, $bound:ident, $operator:tt)),*) => {
        $(
            impl_binary_op!(@loop $op, core::ops::$bound, $operator);
        )*
    };
    (@loop $(($op:ident, $($p:ident)::*, $operator:tt)),*) => {
        $(
            impl_binary_op!($op, $($p)::*, $operator);
        )*

    };

    (@loop $operand:ident, $($p:ident)::*, $op:ident) => {
        operator!($operand, Binary, $op);

        impl<A, B, C> BinOp<A, B> for $operand
        where
            A: $($p)::*<B, Output = C>,
        {
            type Output = C;

            fn eval(&self, lhs: A, rhs: B) -> Self::Output {
                $($p)::*::$op(lhs, rhs)
            }
        }
    };
    (@loop $operand:ident, $($p:ident)::*, $op:tt) => {
        operator!($operand, Binary);

        impl<A, B, C> BinOp<A, B> for $operand
        where
            A: $($p)::*<B, Output = C>,
        {
            type Output = C;

            fn eval(&self, lhs: A, rhs: B) -> Self::Output {
                lhs $op rhs
            }
        }
    };
}

macro_rules! impl_evaluator {
    ($(($operand:ident, $($p:ident)::*, $call:ident)),*) => {
        $(
            impl_evaluator!(@loop $operand, $($p)::*, $call);
        )*
    };
    (@loop $operand:ident, $($p:ident)::*,  $call:ident) => {
        impl<P, A, B, C> Evaluator<P> for $operand
        where
            A: $($p)::*<B, Output = C>,
            P: $crate::ops::Params<Pattern = (A, B)>
        {
            type Output = C;

            fn eval(&self, args: P) -> Self::Output {
                let (lhs, rhs) = args.into_pattern();
                $($p)::*::$call(lhs, rhs)
            }
        }
    };
}

impl_evaluator!(
    (Addition, core::ops::Add, add),
    (Division, core::ops::Div, div),
    (Multiplication, core::ops::Mul, mul),
    (Remainder, core::ops::Rem, rem),
    (Subtraction, core::ops::Sub, sub),
    (Power, num::traits::Pow, pow)
);

impl_binary_op!(std
    (Addition, Add, +),
    (Division, Div, /),
    (Multiplication, Mul, *),
    (Remainder, Rem, %),
    (Subtraction, Sub, -)
);

impl_binary_op!(other: Power, Pow, pow);

impl_binary_op!(
    std(BitAnd, BitAnd, bitand),
    (BitOr, BitOr, bitor),
    (BitXor, BitXor, bitxor),
    (Shl, Shl, shl),
    (Shr, Shr, shr)
);

impl_arith!(
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

    pub fn is_commutative(&self) -> bool {
        match self {
            Arithmetic::Add(_) | Arithmetic::Mul(_) => true,
            _ => false,
        }
    }
}

impl Default for Arithmetic {
    fn default() -> Self {
        Arithmetic::add()
    }
}

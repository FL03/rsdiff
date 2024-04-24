/*
    Appellation: arithmetic <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{BinOp, BinaryAssignOp};
use crate::ops::{Evaluate, OpKind, Operator, Params};
use num::traits::{NumOps, Pow};

macro_rules! impl_binary_op {
    // ($($args:tt),*) => {
    //     impl_binary_op!(@loop $($args),*);

    // };
    ($(($operand:ident, $($p:ident)::*.$op:ident)),*) => {
        $(
            impl_binary_op!(@loop $operand, $($p)::*.$op);
        )*
    };
    ($operand:ident, $($p:ident)::*.$op:ident) => {
        impl_binary_op!(@loop $operand, $($p)::*.$op);
    };
    (std $(($op:ident, $bound:ident, $operator:tt)),*) => {
        $(
            impl_binary_op!(@loop $op, core::ops::$bound, $operator);
        )*
    };
    (@loop $operand:ident, $($p:ident)::*.$op:ident) => {
        operator!($operand<Binary>.$op);
        // impl_evaluator!($operand, $($p)::*.$op);

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
    (@loop $(($op:ident, $($p:ident)::*, $operator:tt)),*) => {
        $(
            impl_binary_op!($op, $($p)::*, $operator);
        )*

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

macro_rules! assign_op {
    ($(($operand:ident, $($p:ident)::*, $op:tt)),*) => {
        $(assign_op!(@loop $operand, $($p)::*, $op);)*
    };
    ($operand:ident, $($p:ident)::*, $op:tt) => {
        assign_op!(@impl $operand, $($p)::*, $op);
    };
    (@impl $operand:ident, $($p:ident)::*, $op:tt) => {
        operator!($operand<Binary>);

        impl<A, B> BinOp<A, B> for $operand
        where
            A: $($p)::*<B>,
        {
            type Output = A;

            fn eval(&self, lhs: A, rhs: B) -> Self::Output {
                let mut lhs = lhs;
                lhs $op rhs;
                lhs
            }
        }

        impl<A, B> BinaryAssignOp<A, B> for $operand
        where
            A: $($p)::*<B>,
        {
            fn eval(&self, mut lhs: A, rhs: B) {
                lhs $op rhs;
            }
        }
    };
}

impl<O, P, A, B> Evaluate<P> for O
where
    O: BinOp<A, B> + Operator,
    P: Params<Pattern = (A, B)>,
{
    type Output = <O as BinOp<A, B>>::Output;

    fn eval(&self, args: P) -> Self::Output {
        let (lhs, rhs) = args.into_pattern();
        BinOp::eval(self, lhs, rhs)
    }
}
impl_binary_op!(
    (Addition, core::ops::Add.add),
    (Division, core::ops::Div.div),
    (Multiplication, core::ops::Mul.mul),
    (Remainder, core::ops::Rem.rem),
    (Subtraction, core::ops::Sub.sub),
    (Power, num::traits::Pow.pow)
);

assign_op!(AddAssign, core::ops::AddAssign, +=);

// impl_binary_op!(
//     (BitAnd, BitAnd.
//     (BitOr, BitOr, |),
//     (BitXor, BitXor, &|),
//     (Shl, Shl, <<),
//     (Shr, Shr, >>)
// );

operator_group!(Arithmetic<Binary> {
    Add(Addition): add,
    Div(Division): div,
    Mul(Multiplication): mul,
    Pow(Power): pow,
    Rem(Remainder): rem,
    Sub(Subtraction): sub
});

operator_group!(ArithmeticAssign<Binary> {
    AddAssign(AddAssign): add_assign
});

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

    pub fn binop<A, B, C>(&self) -> Box<dyn BinOp<A, B, Output = C>>
    where
        A: NumOps<B, C> + Pow<B, Output = C>,
    {
        match *self {
            Arithmetic::Add(op) => Box::new(op),
            Arithmetic::Div(op) => Box::new(op),
            Arithmetic::Mul(op) => Box::new(op),
            Arithmetic::Pow(op) => Box::new(op),
            Arithmetic::Rem(op) => Box::new(op),
            Arithmetic::Sub(op) => Box::new(op),
        }
    }
}

impl Default for Arithmetic {
    fn default() -> Self {
        Arithmetic::add()
    }
}

impl<A, B> BinOp<A, B> for Arithmetic
where
    A: NumOps<B> + Pow<B, Output = A>,
{
    type Output = A;

    fn eval(&self, lhs: A, rhs: B) -> Self::Output {
        match self {
            Arithmetic::Add(op) => BinOp::eval(op, lhs, rhs),
            Arithmetic::Div(op) => BinOp::eval(op, lhs, rhs),
            Arithmetic::Mul(op) => BinOp::eval(op, lhs, rhs),
            Arithmetic::Pow(op) => BinOp::eval(op, lhs, rhs),
            Arithmetic::Rem(op) => BinOp::eval(op, lhs, rhs),
            Arithmetic::Sub(op) => BinOp::eval(op, lhs, rhs),
        }
    }
}

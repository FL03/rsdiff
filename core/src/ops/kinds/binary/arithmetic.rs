/*
    Appellation: arithmetic <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{BinOp, BinaryAssignOp};
use num::traits::{NumAssignOps, NumOps, Pow};

macro_rules! impl_binary_op {
    ($($operand:ident($($p:ident)::*.$op:ident)),*) => {
        $(
            impl_binary_op!(@loop $operand, $($p)::*.$op);
        )*
    };
    (std $(($op:ident, $bound:ident, $operator:tt)),*) => {
        $(
            impl_binary_op!(@loop $op, core::ops::$bound, $operator);
        )*
    };
    (@loop $operand:ident, $($p:ident)::*.$op:ident) => {
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

macro_rules! impl_binary_assign {
    ($($operand:ident($($p:ident)::*.$op:ident)),*) => {
        $(
            impl_binary_assign!(@impl $operand($($p)::*.$op));
        )*
    };
    (@impl $operand:ident($($p:ident)::*.$op:ident)) => {

        impl<A, B> BinOp<A, B> for $operand
        where
            A: $($p)::*<B>,
        {
            type Output = A;

            fn eval(&self, lhs: A, rhs: B) -> Self::Output {
                let mut lhs = lhs;
                $($p)::*::$op(&mut lhs, rhs);
                lhs
            }
        }

        impl<A, B> BinaryAssignOp<A, B> for $operand
        where
            A: $($p)::*<B>,
        {
            fn eval(&self, lhs: &mut A, rhs: B) {
                $($p)::*::$op(lhs, rhs);
            }
        }
    };
}

impl_binary_op!(
    Addition(core::ops::Add.add),
    Division(core::ops::Div.div),
    Multiplication(core::ops::Mul.mul),
    Remainder(core::ops::Rem.rem),
    Subtraction(core::ops::Sub.sub),
    Power(num::traits::Pow.pow)
);

impl_binary_assign!(
    AddAssign(core::ops::AddAssign.add_assign),
    DivAssign(core::ops::DivAssign.div_assign),
    MulAssign(core::ops::MulAssign.mul_assign),
    RemAssign(core::ops::RemAssign.rem_assign),
    SubAssign(core::ops::SubAssign.sub_assign)
);

// impl_binary_op!(
//     (BitAnd, BitAnd.
//     (BitOr, BitOr, |),
//     (BitXor, BitXor, &|),
//     (Shl, Shl, <<),
//     (Shr, Shr, >>)
// );

operations!(Arithmetic<Binary> {
    Add(Addition): add,
    Div(Division): div,
    Mul(Multiplication): mul,
    Pow(Power): pow,
    Rem(Remainder): rem,
    Sub(Subtraction): sub
});

operations!(ArithmeticAssign<Binary> {
    AddAssign(AddAssign): add_assign,
    DivAssign(DivAssign): div_assign,
    MulAssign(MulAssign): mul_assign,
    RemAssign(RemAssign): rem_assign,
    SubAssign(SubAssign): sub_assign
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

impl ArithmeticAssign {
    pub fn new(op: ArithmeticAssign) -> Self {
        op
    }

    pub fn assign_op<A, B>(&self) -> Box<dyn BinaryAssignOp<A, B>>
    where
        A: NumAssignOps<B>,
    {
        match *self {
            ArithmeticAssign::AddAssign(op) => Box::new(op),
            ArithmeticAssign::DivAssign(op) => Box::new(op),
            ArithmeticAssign::MulAssign(op) => Box::new(op),
            ArithmeticAssign::RemAssign(op) => Box::new(op),
            ArithmeticAssign::SubAssign(op) => Box::new(op),
        }
    }

    pub fn bin_op<A, B>(&self) -> Box<dyn BinOp<A, B, Output = A>>
    where
        A: NumAssignOps<B>,
    {
        match *self {
            ArithmeticAssign::AddAssign(op) => Box::new(op),
            ArithmeticAssign::DivAssign(op) => Box::new(op),
            ArithmeticAssign::MulAssign(op) => Box::new(op),
            ArithmeticAssign::RemAssign(op) => Box::new(op),
            ArithmeticAssign::SubAssign(op) => Box::new(op),
        }
    }
}

impl Default for Arithmetic {
    fn default() -> Self {
        Arithmetic::add()
    }
}

impl Default for ArithmeticAssign {
    fn default() -> Self {
        ArithmeticAssign::add_assign()
    }
}

impl<A, B> BinOp<A, B> for Arithmetic
where
    A: NumOps<B> + Pow<B, Output = A>,
{
    type Output = A;

    fn eval(&self, lhs: A, rhs: B) -> Self::Output {
        self.binop().eval(lhs, rhs)
    }
}

impl<A, B> BinOp<A, B> for ArithmeticAssign
where
    A: NumAssignOps<B>,
{
    type Output = A;

    fn eval(&self, lhs: A, rhs: B) -> Self::Output {
        self.bin_op().eval(lhs, rhs)
    }
}

impl<A, B> BinaryAssignOp<A, B> for ArithmeticAssign
where
    A: NumAssignOps<B>,
{
    fn eval(&self, lhs: &mut A, rhs: B) {
        self.assign_op().eval(lhs, rhs)
    }
}

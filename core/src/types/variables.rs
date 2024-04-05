/*
    Appellation: variables <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{BinaryOp, Gradient, Op, UnaryOp};
use crate::specs::{Evaluate, EvaluateMut, EvaluateOnce};
use core::borrow::{Borrow, BorrowMut};
use core::ops::{Neg, Not};
use num::{Num, One, Zero};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Variable<T> {
    name: String,
    operation: Option<Op>,
    pub(crate) value: Option<T>,
}

impl<T> Variable<T> {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            operation: None,
            value: None,
        }
    }

    pub const fn is_expression(&self) -> bool {
        self.operation.is_some()
    }

    pub const fn is_initialized(&self) -> bool {
        self.value.is_some()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn op(&self) -> Option<&Op> {
        self.operation.as_ref()
    }

    pub fn op_mut(&mut self) -> Option<&mut Op> {
        self.operation.as_mut()
    }

    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn set(&mut self, value: T) {
        self.value = Some(value);
    }

    pub fn with_name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_op(mut self, op: impl Into<Op>) -> Self {
        self.operation = Some(op.into());
        self
    }

    pub fn with_value(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }
}

impl<T> Borrow<T> for Variable<T> {
    fn borrow(&self) -> &T {
        self.value.as_ref().unwrap()
    }
}

impl<T> BorrowMut<T> for Variable<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.value.as_mut().unwrap()
    }
}

impl<T> std::fmt::Display for Variable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<T> Evaluate for Variable<T>
where
    T: Copy + Default,
{
    fn eval(&self) -> Self::Output {
        self.value.as_ref().copied().unwrap_or_default()
    }
}
impl<T> EvaluateMut for Variable<T>
where
    T: Default,
{
    fn eval_mut(&mut self) -> Self::Output {
        self.value.take().unwrap_or_default()
    }
}

impl<T> EvaluateOnce for Variable<T>
where
    T: Default,
{
    type Output = T;

    fn eval_once(self) -> Self::Output {
        self.value.unwrap_or_default()
    }
}

impl<T> Gradient<Variable<T>> for Variable<T>
where
    T: Num,
{
    type Gradient = T;

    fn grad(&self, args: Variable<T>) -> Self::Gradient {
        if self.name() == args.name() {
            return T::one();
        }
        T::zero()
    }
}

unsafe impl<T> Send for Variable<T> {}

unsafe impl<T> Sync for Variable<T> {}

impl<T> Neg for Variable<T>
where
    T: Copy + Default + Neg<Output = T>,
{
    type Output = Variable<T>;

    fn neg(self) -> Self::Output {
        let name = format!("-{}", self.name());
        let value = self.eval_once().neg();
        Variable::new(name).with_op(UnaryOp::Neg).with_value(value)
    }
}

impl<'a, T> Neg for &'a Variable<T>
where
    T: Copy + Default + Neg<Output = T>,
{
    type Output = Variable<T>;

    fn neg(self) -> Self::Output {
        let name = format!("-{}", self.name());
        let value = self.eval().neg();
        Variable::new(name).with_op(UnaryOp::Neg).with_value(value)
    }
}

impl<T> Not for Variable<T>
where
    T: Copy + Default + Not<Output = T>,
{
    type Output = Variable<T>;

    fn not(self) -> Self::Output {
        let name = format!("!{}", self.name());
        let value = self.eval_once().not();
        Variable::new(name).with_op(UnaryOp::Not).with_value(value)
    }
}

impl<T> Num for Variable<T>
where
    T: Copy + Default + Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(|value| Variable::new(str).with_value(value))
    }
}

impl<T> One for Variable<T>
where
    T: Copy + Default + One,
{
    fn one() -> Self {
        Variable::new("one").with_value(T::one())
    }
}

impl<T> Zero for Variable<T>
where
    T: Copy + Default + Zero,
{
    fn zero() -> Self {
        Variable::new("0").with_value(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.clone().eval_once().is_zero()
    }
}

macro_rules! impl_std_op {
    ($(($trait:ident, $method:ident, $e:tt)),*) => {
        $(
            impl_std_op!($trait, $method, $e);
        )*
    };
    ($trait:ident, $method:ident, $e:tt) => {
        impl<T> core::ops::$trait for Variable<T>
        where
            T: Copy + Default + core::ops::$trait<Output = T>,
        {
            type Output = Variable<T>;

            fn $method(self, rhs: Variable<T>) -> Self::Output {
                let name = format!("{}", stringify!($method));
                let value = self.eval_once() $e rhs.eval_once();
                Variable::new(name).with_op(BinaryOp::$method()).with_value(value)
            }
        }

        impl<'a, T> core::ops::$trait<&'a Variable<T>> for Variable<T>
        where
            T: Copy + Default + core::ops::$trait<Output = T>,
        {
            type Output = Variable<T>;

            fn $method(self, rhs: &'a Variable<T>) -> Self::Output {
                let name = format!("{}", stringify!($method));
                let value = self.eval_once() $e rhs.eval();
                Variable::new(name).with_op(BinaryOp::$method()).with_value(value)
            }
        }

        impl<'a, T> core::ops::$trait<Variable<T>> for &'a Variable<T>
        where
            T: Copy + Default + core::ops::$trait<Output = T>,
        {
            type Output = Variable<T>;

            fn $method(self, rhs: Variable<T>) -> Self::Output {
                let name = format!("{}", stringify!($method));
                let value = self.eval() $e rhs.eval_once();
                Variable::new(name).with_op(BinaryOp::$method()).with_value(value)
            }
        }

        impl<'a, T> core::ops::$trait<&'a Variable<T>> for &'a Variable<T>
        where
            T: Copy + Default + core::ops::$trait<Output = T>,
        {
            type Output = Variable<T>;

            fn $method(self, rhs: &'a Variable<T>) -> Self::Output {
                let name = format!("{}", stringify!($method));
                let value = self.eval() $e rhs.eval();
                Variable::new(name).with_op(BinaryOp::$method()).with_value(value)
            }
        }

        impl<T> core::ops::$trait<T> for Variable<T>
        where
            T: Copy + Default + core::ops::$trait<Output = T>,
        {
            type Output = Self;

            fn $method(self, rhs: T) -> Self::Output {
                let name = format!("{}", stringify!($method));
                let value = self.eval_once() $e rhs;
                Variable::new(name).with_op(BinaryOp::$method()).with_value(value)
            }
        }
    };
}

impl_std_op!((Add, add, +), (Div, div, /), (Mul, mul, *), (Rem, rem, %), (Sub, sub, -));

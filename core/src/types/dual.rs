/*
    Appellation: dual <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Dual
//!
//! Dual numbers are a type of hypercomplex number which are expressions of
//! the form:
//!     Dual => z = a + be
//!     where
//!         a, b, e are real numbers
//!         e != 0
//!         e^2 = 0

use crate::prelude::{EvaluateOnce, Gradient};
use core::ops::{self, Neg, Not};
use num::{Num, One, Zero};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
pub struct Dual<T> {
    dual: T,
    value: T,
}

impl<T> Dual<T> {
    pub fn new(value: T, dual: T) -> Self {
        Self { dual, value }
    }

    pub fn from_real(value: T) -> Self
    where
        T: Default,
    {
        Self::new(value, T::default())
    }

    pub fn dual(&self) -> &T {
        &self.dual
    }

    pub fn dual_mut(&mut self) -> &mut T {
        &mut self.dual
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> std::fmt::Display for Dual<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.value, self.dual)
    }
}

impl<T> EvaluateOnce for Dual<T> {
    type Output = T;

    fn eval_once(self) -> Self::Output {
        self.value
    }
}

impl<T> Gradient<T> for Dual<T>
where
    T: Default + Gradient<T>,
{
    type Gradient = Dual<T>;

    fn grad(&self, _: T) -> Self::Gradient {
        Dual::from_real(T::default())
    }
}

impl<T> Neg for Dual<T>
where
    T: Neg<Output = T>,
{
    type Output = Dual<T>;

    fn neg(self) -> Self::Output {
        Dual::new(-self.value, -self.dual)
    }
}

impl<T> Not for Dual<T>
where
    T: Not<Output = T>,
{
    type Output = Dual<T>;

    fn not(self) -> Self::Output {
        Dual::new(!self.value, !self.dual)
    }
}

unsafe impl<T> Send for Dual<T> {}

unsafe impl<T> Sync for Dual<T> {}

impl<T> From<T> for Dual<T>
where
    T: Default,
{
    fn from(value: T) -> Self {
        Self::from_real(value)
    }
}

impl<T> ops::Div for Dual<T>
where
    T: Copy + ops::Div<Output = T> + ops::Mul<Output = T> + ops::Sub<Output = T>,
{
    type Output = Dual<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Dual::new(
            self.value / rhs.value,
            (self.dual * rhs.value - self.value * rhs.dual) / (rhs.value * rhs.value),
        )
    }
}

impl<T> ops::Div<T> for Dual<T>
where
    T: Copy + ops::Div<Output = T>,
{
    type Output = Dual<T>;

    fn div(self, rhs: T) -> Self::Output {
        Dual::new(self.value / rhs, self.dual / rhs)
    }
}

impl<T> ops::DivAssign for Dual<T>
where
    T: Copy + ops::DivAssign + num::traits::NumOps,
{
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
        self.dual = (self.dual * rhs.value - self.value * rhs.dual) / (rhs.value * rhs.value);
    }
}

impl<T> ops::DivAssign<T> for Dual<T>
where
    T: Copy + ops::DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        self.value /= rhs;
        self.dual /= rhs;
    }
}

impl<T> Num for Dual<T>
where
    T: Copy + Default + Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(Dual::from_real)
    }
}

impl<T> One for Dual<T>
where
    T: Copy + One + PartialEq + ops::Add<Output = T>,
{
    fn one() -> Self {
        Dual::new(T::one(), T::one())
    }

    fn is_one(&self) -> bool {
        self.value.is_one()
    }
}

impl<T> Zero for Dual<T>
where
    T: Copy + Zero,
{
    fn zero() -> Self {
        Dual::new(T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

macro_rules! impl_binary_op {
    ($(($op:ident, $method:ident, $e:tt)),*) => {
        $(impl_binary_op!($op, $method, $e);)*
    };
    ($trait:ident, $method:ident, $e:tt) => {
        impl<T> ops::$trait<Dual<T>> for Dual<T>
        where
            T: Copy + ops::$trait<T, Output = T>,
        {
            type Output = Dual<T>;

            fn $method(self, rhs: Self) -> Self::Output {
                let real = self.value $e rhs.value;
                let dual = self.dual $e rhs.dual;
                Dual::new(real, dual)
            }
        }

        impl<'a, T> ops::$trait<&'a Dual<T>> for Dual<T>
        where
            T: Copy + ops::$trait<T, Output = T>,
        {
            type Output = Dual<T>;

            fn $method(self, rhs: &'a Dual<T>) -> Self::Output {
                let real = self.value $e rhs.value;
                let dual = self.dual $e rhs.dual;
                Dual::new(real, dual)
            }
        }

        impl<'a, T> ops::$trait<Dual<T>> for &'a Dual<T>
        where
            T: Copy + ops::$trait<T, Output = T>,
        {
            type Output = Dual<T>;

            fn $method(self, rhs: Dual<T>) -> Self::Output {
                let real = self.value $e rhs.value;
                let dual = self.dual $e rhs.dual;
                Dual::new(real, dual)
            }
        }

        impl<'a, T> ops::$trait<&'a Dual<T>> for &'a Dual<T>
        where
            T: Copy + ops::$trait<T, Output = T>,
        {
            type Output = Dual<T>;

            fn $method(self, rhs: &'a Dual<T>) -> Self::Output {
                let real = self.value $e rhs.value;
                let dual = self.dual $e rhs.dual;
                Dual::new(real, dual)
            }
        }

        impl<T> ops::$trait<T> for Dual<T>
        where
            T: Copy + ops::$trait<Output = T>,
        {
            type Output = Dual<T>;

            fn $method(self, rhs: T) -> Self::Output {
                let real = self.value $e rhs;
                Dual::new(real, self.dual)
            }
        }

        impl<'a, T> ops::$trait<T> for &'a Dual<T>
        where
            T: Copy + ops::$trait<T, Output = T>,
        {
            type Output = Dual<T>;

            fn $method(self, rhs: T) -> Self::Output {
                let real = self.value $e rhs;
                Dual::new(real, self.dual)
            }
        }
    };
}

macro_rules! impl_assign_op {
    ($(($op:ident, $method:ident, $e:tt)),*) => {
        $(impl_assign_op!($op, $method, $e);)*
    };
    ($trait:ident, $method:ident, $e:tt) => {
        impl<T> ops::$trait<Dual<T>> for Dual<T>
        where
            T: Copy + ops::$trait<T>,
        {
            fn $method(&mut self, rhs: Self) {
                self.value $e rhs.value;
                self.dual $e rhs.dual;
            }
        }

        impl<'a, T> ops::$trait<&'a Dual<T>> for Dual<T>
        where
            T: Copy + ops::$trait<T>,
        {
            fn $method(&mut self, rhs: &'a Dual<T>) {
                self.value $e rhs.value;
                self.dual $e rhs.dual;
            }
        }

        impl<T> ops::$trait<T> for Dual<T>
        where
            T: Copy + ops::$trait,
        {
            fn $method(&mut self, rhs: T) {
                self.value $e rhs;
            }
        }
    };
}

impl_binary_op!((Add, add, +), (Mul, mul, *), (Rem, rem, %), (Sub, sub, -));
impl_assign_op!((AddAssign, add_assign, +=), (MulAssign, mul_assign, *=), (RemAssign, rem_assign, %=), (SubAssign, sub_assign, -=));

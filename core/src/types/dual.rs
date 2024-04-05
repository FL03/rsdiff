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
    real: T,
}

impl<T> Dual<T> {
    pub fn new(real: T, dual: T) -> Self {
        Self { dual, real }
    }

    pub fn real(real: T) -> Self
    where
        T: Default,
    {
        Self {
            dual: T::default(),
            real,
        }
    }

    pub fn value(&self) -> &T {
        &self.real
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.real
    }
}

impl<T> std::fmt::Display for Dual<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.real, self.dual)
    }
}

impl<T> EvaluateOnce for Dual<T> {
    type Output = T;

    fn eval_once(self) -> Self::Output {
        self.real
    }
}

impl<T> Gradient<T> for Dual<T>
where
    T: Default + Gradient<T>,
{
    type Gradient = Dual<T>;

    fn grad(&self, _: T) -> Self::Gradient {
        Dual::real(T::default())
    }
}

impl<T> Neg for Dual<T>
where
    T: Neg<Output = T>,
{
    type Output = Dual<T>;

    fn neg(self) -> Self::Output {
        Dual::new(-self.real, -self.dual)
    }
}

impl<T> Not for Dual<T>
where
    T: Not<Output = T>,
{
    type Output = Dual<T>;

    fn not(self) -> Self::Output {
        Dual::new(!self.real, !self.dual)
    }
}

unsafe impl<T> Send for Dual<T> {}

unsafe impl<T> Sync for Dual<T> {}

impl<T> From<T> for Dual<T>
where
    T: Default,
{
    fn from(value: T) -> Self {
        Self::real(value)
    }
}

impl<T> ops::Div for Dual<T>
where
    T: Copy + ops::Div<Output = T> + ops::Mul<Output = T> + ops::Sub<Output = T>,
{
    type Output = Dual<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Dual::new(
            self.real / rhs.real,
            (self.dual * rhs.real - self.real * rhs.dual) / (rhs.real * rhs.real),
        )
    }
}

impl<T> ops::Div<T> for Dual<T>
where
    T: Copy + ops::Div<Output = T>,
{
    type Output = Dual<T>;

    fn div(self, rhs: T) -> Self::Output {
        Dual::new(self.real / rhs, self.dual / rhs)
    }
}

impl<T> ops::DivAssign for Dual<T>
where
    T: Copy + ops::DivAssign + num::traits::NumOps,
{
    fn div_assign(&mut self, rhs: Self) {
        self.real /= rhs.real;
        self.dual = (self.dual * rhs.real - self.real * rhs.dual) / (rhs.real * rhs.real);
    }
}

impl<T> ops::DivAssign<T> for Dual<T>
where
    T: Copy + ops::DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        self.real /= rhs;
        self.dual /= rhs;
    }
}

impl<T> Num for Dual<T>
where
    T: Copy + Default + Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(Dual::real)
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
        self.real.is_one()
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
        self.real.is_zero()
    }
}

macro_rules! impl_binary_op {
    ($(($op:ident, $method:ident, $e:tt)),*) => {
        $(impl_binary_op!($op, $method, $e);)*
    };
    ($trait:ident, $method:ident, $e:tt) => {
        impl<T> std::ops::$trait<Dual<T>> for Dual<T>
        where
            T: Copy + std::ops::$trait<T, Output = T>,
        {
            type Output = Dual<T>;

            fn $method(self, rhs: Self) -> Self::Output {
                let real = self.real $e rhs.real;
                let dual = self.dual $e rhs.dual;
                Dual::new(real, dual)
            }
        }

        impl<'a, T> std::ops::$trait<&'a Dual<T>> for Dual<T>
        where
            T: Copy + std::ops::$trait<T, Output = T>,
        {
            type Output = Dual<T>;

            fn $method(self, rhs: &'a Dual<T>) -> Self::Output {
                let real = self.real $e rhs.real;
                let dual = self.dual $e rhs.dual;
                Dual::new(real, dual)
            }
        }

        impl<'a, T> std::ops::$trait<Dual<T>> for &'a Dual<T>
        where
            T: Copy + std::ops::$trait<T, Output = T>,
        {
            type Output = Dual<T>;

            fn $method(self, rhs: Dual<T>) -> Self::Output {
                let real = self.real $e rhs.real;
                let dual = self.dual $e rhs.dual;
                Dual::new(real, dual)
            }
        }

        impl<'a, T> std::ops::$trait<&'a Dual<T>> for &'a Dual<T>
        where
            T: Copy + std::ops::$trait<T, Output = T>,
        {
            type Output = Dual<T>;

            fn $method(self, rhs: &'a Dual<T>) -> Self::Output {
                let real = self.real $e rhs.real;
                let dual = self.dual $e rhs.dual;
                Dual::new(real, dual)
            }
        }

        impl<T> std::ops::$trait<T> for Dual<T>
        where
            T: Copy + std::ops::$trait<Output = T>,
        {
            type Output = Dual<T>;

            fn $method(self, rhs: T) -> Self::Output {
                let real = self.real $e rhs;
                Dual::new(real, self.dual)
            }
        }

        impl<'a, T> std::ops::$trait<T> for &'a Dual<T>
        where
            T: Copy + std::ops::$trait<T, Output = T>,
        {
            type Output = Dual<T>;

            fn $method(self, rhs: T) -> Self::Output {
                let real = self.real $e rhs;
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
        impl<T> std::ops::$trait<Dual<T>> for Dual<T>
        where
            T: Copy + std::ops::$trait<T>,
        {
            fn $method(&mut self, rhs: Self) {
                self.real $e rhs.real;
                self.dual $e rhs.dual;
            }
        }

        impl<'a, T> std::ops::$trait<&'a Dual<T>> for Dual<T>
        where
            T: Copy + std::ops::$trait<T>,
        {
            fn $method(&mut self, rhs: &'a Dual<T>) {
                self.real $e rhs.real;
                self.dual $e rhs.dual;
            }
        }

        impl<T> std::ops::$trait<T> for Dual<T>
        where
            T: Copy + std::ops::$trait,
        {
            fn $method(&mut self, rhs: T) {
                self.real $e rhs;
            }
        }
    };
}

impl_binary_op!((Add, add, +), (Mul, mul, *), (Rem, rem, %), (Sub, sub, -));
impl_assign_op!((AddAssign, add_assign, +=), (MulAssign, mul_assign, *=), (RemAssign, rem_assign, %=), (SubAssign, sub_assign, -=));

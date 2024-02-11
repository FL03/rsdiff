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

use crate::ops::{Evaluate, Gradient};
use num::{Num, One, Zero};
use serde::{Deserialize, Serialize};
use std::marker::ConstParamTy;
use std::ops::{self, Neg, Not};

#[derive(
    Clone,
    ConstParamTy,
    Copy,
    Debug,
    Default,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct Dual<T> {
    dual: T,
    real: T,
}

impl<T> Dual<T> {
    pub fn new(real: T, dual: T) -> Self {
        Self { real, dual }
    }

    pub fn real(value: T) -> Self
    where
        T: Default,
    {
        Self::new(value, T::default())
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

impl<T> Evaluate for Dual<T> {
    type Output = T;

    fn eval(self) -> Self::Output {
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

impl<T> From<T> for Dual<T>
where
    T: Default,
{
    fn from(value: T) -> Self {
        Self::real(value)
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

impl<T> ops::Add for Dual<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Dual<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Dual::new(self.real + rhs.real, self.dual + rhs.dual)
    }
}

impl<T> ops::Add<T> for Dual<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Dual<T>;

    fn add(self, rhs: T) -> Self::Output {
        Dual::new(self.real + rhs, self.dual)
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
    T: Copy + ops::Div<Output = T> + ops::Mul<Output = T> + ops::Sub<Output = T>,
{
    type Output = Dual<T>;

    fn div(self, rhs: T) -> Self::Output {
        Dual::new(self.real / rhs, self.dual / rhs)
    }
}

impl<T> ops::Mul for Dual<T>
where
    T: ops::Mul<Output = T> + ops::Add<Output = T> + Copy,
{
    type Output = Dual<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Dual::new(
            self.real * rhs.real,
            self.dual * rhs.real + rhs.dual * self.real,
        )
    }
}

impl<T> ops::Mul<T> for Dual<T>
where
    T: ops::Mul<Output = T>,
{
    type Output = Dual<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Dual::new(self.real * rhs, self.dual)
    }
}

impl<T> ops::Rem for Dual<T>
where
    T: ops::Rem<Output = T>,
{
    type Output = Dual<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        Dual::new(self.real % rhs.real, self.dual % rhs.dual)
    }
}

impl<T> ops::Rem<T> for Dual<T>
where
    T: ops::Rem<Output = T>,
{
    type Output = Dual<T>;

    fn rem(self, rhs: T) -> Self::Output {
        Dual::new(self.real % rhs, self.dual)
    }
}

impl<T> ops::Sub for Dual<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Dual<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Dual::new(self.real - rhs.real, self.dual - rhs.dual)
    }
}

impl<T> ops::Sub<T> for Dual<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Dual<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Dual::new(self.real - rhs, self.dual)
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
    T: Zero,
{
    fn zero() -> Self {
        Dual::new(T::zero(), T::zero())
    }

    fn is_zero(&self) -> bool {
        self.real.is_zero()
    }
}

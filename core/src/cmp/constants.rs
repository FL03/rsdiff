/*
    Appellation: constants <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::{Evaluate, Gradient};
use num::{Num, One, Zero};
use serde::{Deserialize, Serialize};
use std::ops::{self, Deref, DerefMut, Neg, Not};

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[repr(transparent)]
pub struct Constant<T>(pub T);

impl<T> Constant<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &T {
        &self.0
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> AsRef<T> for Constant<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Constant<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Deref for Constant<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Constant<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> std::fmt::Display for Constant<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> Evaluate<()> for Constant<T>
where
    T: Clone,
{
    type Output = T;

    fn eval(&self) -> Self::Output {
        self.0.clone()
    }
}

impl<T> Gradient<T> for Constant<T>
where
    T: Default + Gradient<T>,
{
    type Gradient = Constant<T>;

    fn grad(&self, _: T) -> Self::Gradient {
        Constant::new(T::default())
    }
}

impl<T> From<T> for Constant<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> Neg for Constant<T>
where
    T: Neg<Output = T>,
{
    type Output = Constant<T>;

    fn neg(self) -> Self::Output {
        Constant::new(-self.0)
    }
}

impl<T> Not for Constant<T>
where
    T: Not<Output = T>,
{
    type Output = Constant<T>;

    fn not(self) -> Self::Output {
        Constant::new(!self.0)
    }
}

unsafe impl<T> Send for Constant<T> {}

unsafe impl<T> Sync for Constant<T> {}

impl<T> ops::Add for Constant<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Constant<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Constant::new(self.0 + rhs.0)
    }
}

impl<T> ops::Add<T> for Constant<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Constant<T>;

    fn add(self, rhs: T) -> Self::Output {
        Constant::new(self.0 + rhs)
    }
}

impl<T> ops::Div for Constant<T>
where
    T: ops::Div<Output = T>,
{
    type Output = Constant<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Constant::new(self.0 / rhs.0)
    }
}

impl<T> ops::Div<T> for Constant<T>
where
    T: ops::Div<Output = T>,
{
    type Output = Constant<T>;

    fn div(self, rhs: T) -> Self::Output {
        Constant::new(self.0 / rhs)
    }
}

impl<T> ops::Mul for Constant<T>
where
    T: ops::Mul<Output = T>,
{
    type Output = Constant<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Constant::new(self.0 * rhs.0)
    }
}

impl<T> ops::Mul<T> for Constant<T>
where
    T: ops::Mul<Output = T>,
{
    type Output = Constant<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Constant::new(self.0 * rhs)
    }
}

impl<T> ops::Rem for Constant<T>
where
    T: ops::Rem<Output = T>,
{
    type Output = Constant<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        Constant::new(self.0 % rhs.0)
    }
}

impl<T> ops::Rem<T> for Constant<T>
where
    T: ops::Rem<Output = T>,
{
    type Output = Constant<T>;

    fn rem(self, rhs: T) -> Self::Output {
        Constant::new(self.0 % rhs)
    }
}

impl<T> ops::Sub for Constant<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Constant<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Constant::new(self.0 - rhs.0)
    }
}

impl<T> ops::Sub<T> for Constant<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Constant<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Constant::new(self.0 - rhs)
    }
}

impl<T> Num for Constant<T>
where
    T: Num,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        T::from_str_radix(str, radix).map(Constant::new)
    }
}

impl<T> One for Constant<T>
where
    T: One + PartialEq,
{
    fn one() -> Self {
        Constant::new(T::one())
    }

    fn is_one(&self) -> bool {
        self.0.is_one()
    }
}

impl<T> Zero for Constant<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Constant::new(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

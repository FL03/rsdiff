/*
    Appellation: constants <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use crate::prelude::{Evaluate, Gradient};
use num::{Num, One, Zero};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut, Neg, Not};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
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

    pub fn into_value(self) -> T {
        self.0
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

impl<T> Evaluate for Constant<T> {
    type Output = T;

    fn eval(self) -> Self::Output {
        self.0
    }
}

impl<T> Gradient<T> for Constant<T>
where
    T: Default,
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

impl<T> Borrow<T> for Constant<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> BorrowMut<T> for Constant<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

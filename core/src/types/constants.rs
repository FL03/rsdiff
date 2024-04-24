/*
    Appellation: constants <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{EvalOnce, Gradient};
use core::borrow::{Borrow, BorrowMut};
use core::ops::{Deref, DerefMut, Neg, Not};
use num::{Num, One, Zero};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize,))]
#[repr(C)]
pub struct Constant<T>(pub(crate) T);

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

macro_rules! fmt_const {
    ($($name:ident($($args:tt)*)),*) => {
        $(
            fmt_const!(@impl $name($($args)*));
        )*
    };
    (@impl $name:ident($($args:tt)*)) => {
        impl<T> core::fmt::$name for Constant<T> where T: core::fmt::$name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, $($args)*, self.0)
            }
        }
    };
}

fmt_const!(
    Binary("{:b}"),
    Display("{}"),
    LowerExp("{:e}"),
    LowerHex("{:x}"),
    Octal("{:o}"),
    UpperExp("{:E}"),
    UpperHex("{:X}")
);

impl<T> EvalOnce for Constant<T> {
    type Output = T;

    fn eval_once(self) -> Self::Output {
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

macro_rules! impl_binary_op {
    ($(($bound:ident, $method:ident, $e:tt)),*) => {
        $(
            impl_binary_op!($bound, $method, $e);
        )*
    };
    ($bound:ident, $fn:ident, $e:tt) => {
        impl<T> core::ops::$bound for Constant<T>
        where
            T: core::ops::$bound<T, Output = T>,
        {
            type Output = Constant<T>;

            fn $fn(self, rhs: Constant<T>) -> Self::Output {
                Constant(self.0 $e rhs.0)
            }
        }

        impl<'a, T> core::ops::$bound<&'a Constant<T>> for Constant<T>
        where
            T: Copy + core::ops::$bound<Output = T>,
        {
            type Output = Constant<T>;

            fn $fn(self, rhs: &'a Constant<T>) -> Self::Output {
                Constant(self.0 $e rhs.0)
            }
        }

        impl<'a, T> core::ops::$bound<Constant<T>> for &'a Constant<T>
        where
            T: Copy + core::ops::$bound<Output = T>,
        {
            type Output = Constant<T>;

            fn $fn(self, rhs: Constant<T>) -> Self::Output {
                Constant(self.0 $e rhs.0)
            }
        }

        impl<'a, T> core::ops::$bound<&'a Constant<T>> for &'a Constant<T>
        where
            T: Copy + core::ops::$bound<Output = T>,
        {
            type Output = Constant<T>;

            fn $fn(self, rhs: &'a Constant<T>) -> Self::Output {
                Constant(self.0 $e rhs.0)
            }
        }

        impl<T> core::ops::$bound<T> for Constant<T>
        where
            T: core::ops::$bound<Output = T>,
        {
            type Output = Self;

            fn $fn(self, rhs: T) -> Self::Output {
                Constant(self.0 $e rhs)
            }
        }

        impl<'a, T> core::ops::$bound<T> for &'a Constant<T>
        where
            T: Copy + core::ops::$bound<Output = T>,
        {
            type Output = Constant<T>;

            fn $fn(self, rhs: T) -> Self::Output {
                Constant(self.0 $e rhs)
            }
        }
    };
}

impl_binary_op!((Add, add, +), (Div, div, /), (Mul, mul, *), (Rem, rem, %), (Sub, sub, -));

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

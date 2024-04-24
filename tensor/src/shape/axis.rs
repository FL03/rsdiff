/*
   Appellation: axis <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::ops::Deref;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub trait IntoAxis {
    fn into_axis(self) -> Axis;
}

impl IntoAxis for usize {
    fn into_axis(self) -> Axis {
        Axis::new(self)
    }
}

/// An [Axis] is used to represent a dimension in a tensor.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Axis(pub(crate) usize);

impl Axis {
    pub fn new(axis: usize) -> Self {
        Self(axis)
    }

    pub fn into_inner(self) -> usize {
        self.0
    }

    pub fn axis(&self) -> usize {
        self.0
    }

    pub fn dec(&self) -> Axis {
        self - 1
    }

    pub fn inc(&self) -> Axis {
        self + 1
    }
}

impl AsRef<usize> for Axis {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

impl Deref for Axis {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for Axis {
    fn from(axis: usize) -> Self {
        Axis(axis)
    }
}

impl From<Axis> for usize {
    fn from(axis: Axis) -> Self {
        axis.0
    }
}

macro_rules! impl_std_ops {
    ($(($trait:tt, $method:ident, $e:tt)),*) => {
        $(
           impl_std_ops!($trait, $method, $e);
        )*
    };
    ($trait:tt, $method:ident, $e:tt) => {
        impl core::ops::$trait<usize> for Axis {
            type Output = Axis;

            fn $method(self, rhs: usize) -> Self::Output {
                Axis(self.0 $e rhs)
            }
        }

        impl<'a> core::ops::$trait<usize> for &'a Axis {
            type Output = Axis;

            fn $method(self, rhs: usize) -> Self::Output {
                Axis(self.0 $e rhs)
            }
        }

        impl core::ops::$trait for Axis {
            type Output = Axis;

            fn $method(self, rhs: Axis) -> Self::Output {
                Axis(self.0 $e rhs.0)
            }
        }

        impl<'a> core::ops::$trait<Axis> for &'a Axis {
            type Output = Axis;

            fn $method(self, rhs: Axis) -> Self::Output {
                Axis(self.0 $e rhs.0)
            }
        }

        impl<'a> core::ops::$trait<&'a Axis> for Axis {
            type Output = Axis;

            fn $method(self, rhs: &'a Axis) -> Self::Output {
                Axis(self.0 $e rhs.0)
            }
        }

        impl<'a> core::ops::$trait<&'a Axis> for &'a Axis {
            type Output = Axis;

            fn $method(self, rhs: &'a Axis) -> Self::Output {
                Axis(self.0 $e rhs.0)
            }
        }
    };
}

impl_std_ops!((Add, add, +), (Sub, sub, -), (Mul, mul, *), (Div, div, /), (Rem, rem, %));

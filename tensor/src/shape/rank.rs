/*
   Appellation: rank <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Rank
//!
//! The rank of a n-dimensional array describes the number of dimensions
use core::borrow::Borrow;
use core::ops::{Deref, DerefMut, Not};
use num::traits::{Num, One, Zero};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub trait IntoRank {
    fn into_rank(self) -> Rank;
}

impl IntoRank for usize {
    fn into_rank(self) -> Rank {
        Rank::new(self)
    }
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Rank(pub(crate) usize);

impl Rank {
    pub fn new(rank: usize) -> Self {
        Self(rank)
    }

    pub const fn scalar() -> Self {
        Self(0)
    }

    pub fn into_inner(self) -> usize {
        self.0
    }

    pub fn is_scalar(&self) -> bool {
        self.0 == 0
    }

    pub fn rank(&self) -> usize {
        self.0
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<usize> for Rank {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

impl AsMut<usize> for Rank {
    fn as_mut(&mut self) -> &mut usize {
        &mut self.0
    }
}

impl Borrow<usize> for Rank {
    fn borrow(&self) -> &usize {
        &self.0
    }
}

impl Deref for Rank {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Rank {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<usize> for Rank {
    fn from(rank: usize) -> Self {
        Self(rank)
    }
}

impl From<Rank> for usize {
    fn from(rank: Rank) -> Self {
        rank.0
    }
}

unsafe impl Send for Rank {}

unsafe impl Sync for Rank {}

impl_partial_eq!(Rank -> 0: [usize]);

macro_rules! impl_std_ops {
    ($(($trait:tt, $method:ident, $e:tt)),*) => {
        $(
           impl_std_ops!($trait, $method, $e);
        )*
    };
    ($trait:tt, $method:ident, $e:tt) => {
        impl core::ops::$trait<usize> for Rank {
            type Output = Rank;

            fn $method(self, rhs: usize) -> Self::Output {
                let rank = self.0 $e rhs;
                Rank(rank)
            }
        }

        impl<'a> core::ops::$trait<usize> for &'a Rank {
            type Output = Rank;

            fn $method(self, rhs: usize) -> Self::Output {
                let rank = self.0 $e rhs;
                Rank(rank)
            }
        }

        impl core::ops::$trait for Rank {
            type Output = Rank;

            fn $method(self, rhs: Rank) -> Self::Output {
                let rank = self.0 $e rhs.0;
                Rank(rank)
            }
        }

        impl<'a> core::ops::$trait<Rank> for &'a Rank {
            type Output = Rank;

            fn $method(self, rhs: Rank) -> Self::Output {
                let rank = self.0 $e rhs.0;
                Rank(rank)
            }
        }

        impl<'a> core::ops::$trait<&'a Rank> for Rank {
            type Output = Rank;

            fn $method(self, rhs: &'a Rank) -> Self::Output {
                let rank = self.0 $e rhs.0;
                Rank(rank)
            }
        }

        impl<'a> core::ops::$trait<&'a Rank> for &'a Rank {
            type Output = Rank;

            fn $method(self, rhs: &'a Rank) -> Self::Output {
                let rank = self.0 $e rhs.0;
                Rank(rank)
            }
        }
    };
}

impl_std_ops!((Add, add, +), (Sub, sub, -), (Mul, mul, *), (Div, div, /), (Rem, rem, %));

impl Not for Rank {
    type Output = Rank;

    fn not(self) -> Self::Output {
        Rank(!self.0)
    }
}

impl Num for Rank {
    type FromStrRadixErr = <usize as Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        usize::from_str_radix(str, radix).map(Rank)
    }
}

impl One for Rank {
    fn one() -> Self {
        Self(1)
    }
}

impl Zero for Rank {
    fn zero() -> Self {
        Self(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

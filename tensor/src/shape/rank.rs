/*
   Appellation: rank <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Rank
//!
//! The rank of a n-dimensional array describes the number of dimensions
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};

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
pub struct Rank(pub usize);

impl Rank {
    pub fn new(rank: usize) -> Self {
        Self(rank)
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

macro_rules! impl_std_ops {
    ($trait:tt, $method:ident, $e:tt) => {
        impl std::ops::$trait<usize> for Rank {
                type Output = usize;

                fn $method(self, rhs: usize) -> Self::Output {
                    self.0 $e rhs
                }
            }

        impl std::ops::$trait<Rank> for Rank {
            type Output = usize;

            fn $method(self, rhs: Rank) -> Self::Output {
                self.0 $e rhs.0
            }
        }

        impl<'a> std::ops::$trait<Rank> for &'a Rank {
            type Output = usize;

            fn $method(self, rhs: Rank) -> Self::Output {
                self.0 $e rhs.0
            }
        }

        impl<'a> std::ops::$trait<&'a Rank> for Rank {
            type Output = usize;

            fn $method(self, rhs: &'a Rank) -> Self::Output {
                self.0 $e rhs.0
            }
        }

        impl<'a> std::ops::$trait<&'a Rank> for &'a Rank {
            type Output = usize;

            fn $method(self, rhs: &'a Rank) -> Self::Output {
                self.0 $e rhs.0
            }
        }
    };
    (many: $(($trait:tt, $method:ident, $e:tt)),*) => {
        $(
           impl_std_ops!($trait, $method, $e);
        )*
    };
}

impl_std_ops!(many: (Add, add, +), (Sub, sub, -), (Mul, mul, *), (Div, div, /), (Rem, rem, %));

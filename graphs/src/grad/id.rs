/*
    Appellation: id <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::id::Id;
use core::fmt;
use core::marker::PhantomData;
use core::ops::Deref;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
pub struct GradientId<T> {
    pub(crate) inner: Id,
    ptr: PhantomData<T>,
}

impl<T> GradientId<T> {
    pub fn new(inner: Id) -> Self {
        Self {
            inner,
            ptr: PhantomData,
        }
    }

    pub fn into_inner(self) -> Id {
        self.inner
    }
}

impl<T> fmt::Display for GradientId<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl<T> Deref for GradientId<T> {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> From<Id> for GradientId<T> {
    fn from(id: Id) -> Self {
        Self::new(id)
    }
}

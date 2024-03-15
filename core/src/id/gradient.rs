/*
    Appellation: gradient <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Id;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::ops::Deref;

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

impl<T> std::fmt::Display for GradientId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.inner)
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

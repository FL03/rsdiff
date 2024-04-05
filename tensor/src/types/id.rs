/*
    Appellation: id <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Tensor Id
//!
//!
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
pub struct TensorId(usize);

impl TensorId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        Self(COUNTER.fetch_add(1, Relaxed))
    }

    pub fn next(&self) -> Self {
        Self::new()
    }

    pub fn set(&mut self, id: usize) {
        self.0 = id;
    }

    pub fn get(&self) -> usize {
        self.0
    }

    pub fn into_inner(self) -> usize {
        self.0
    }
}

impl AsRef<usize> for TensorId {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

impl AsMut<usize> for TensorId {
    fn as_mut(&mut self) -> &mut usize {
        &mut self.0
    }
}

impl Default for TensorId {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for TensorId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TensorId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for TensorId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for TensorId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl From<TensorId> for usize {
    fn from(id: TensorId) -> Self {
        id.0
    }
}

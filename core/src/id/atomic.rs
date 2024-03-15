/*
    Appellation: atomic <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Atomic Id
//!
//!
use crate::id::Identifier;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[repr(transparent)]
pub struct AtomicId(usize);

impl AtomicId {
    pub fn new() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        Self(COUNTER.fetch_add(1, Relaxed))
    }

    pub fn next(&self) -> Self {
        Self::new()
    }

    pub fn set(&mut self, id: usize) {
        self.0 = id;
    }

    pub const fn get(&self) -> usize {
        self.0
    }

    pub fn into_inner(self) -> usize {
        self.0
    }
}

impl AsRef<usize> for AtomicId {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

impl AsMut<usize> for AtomicId {
    fn as_mut(&mut self) -> &mut usize {
        &mut self.0
    }
}

impl Default for AtomicId {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for AtomicId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AtomicId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Identifier for AtomicId {}

impl std::fmt::Display for AtomicId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for AtomicId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl From<AtomicId> for usize {
    fn from(id: AtomicId) -> Self {
        id.0
    }
}

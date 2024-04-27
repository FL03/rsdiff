/*
    Appellation: atomic <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::borrow::{Borrow, BorrowMut};
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicUsize, Ordering::Relaxed};

///
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(C)]
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

impl Borrow<usize> for AtomicId {
    fn borrow(&self) -> &usize {
        &self.0
    }
}

impl BorrowMut<usize> for AtomicId {
    fn borrow_mut(&mut self) -> &mut usize {
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

macro_rules! fmt_atomic {
    ($($trait:ident($($fmt:tt)*)),*) => {
        $(
            fmt_atomic!(@impl $trait($($fmt)*));
        )*
    };
    (@impl $trait:ident($($fmt:tt)*)) => {
        impl core::fmt::$trait for AtomicId {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, $($fmt)*, self.0)
            }
        }
    };
}

fmt_atomic! {
    Binary("{:b}"),
    Debug("{:?}"),
    Display("{}"),
    LowerExp("{:e}"),
    LowerHex("{:x}"),
    Octal("{:o}"),
    UpperExp("{:E}"),
    UpperHex("{:X}")
}

impl<S> PartialEq<S> for AtomicId
where
    usize: PartialEq<S>,
{
    fn eq(&self, other: &S) -> bool {
        self.0.eq(other)
    }
}

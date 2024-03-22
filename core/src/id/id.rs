/*
    Appellation: id <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::AtomicId;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
pub struct Id {
    id: AtomicId,
    index: usize,
}

impl Id {
    pub fn new(index: usize) -> Self {
        Self {
            id: AtomicId::new(),
            index,
        }
    }

    pub fn id(&self) -> usize {
        *self.id
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub(crate) fn next_index(&self) -> Self {
        Self {
            id: self.id,
            index: self.index() + 1,
        }
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{}.{}", self.index(), self.id)
        } else {
            write!(f, "{}", self.index())
        }
    }
}

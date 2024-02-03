/*
    Appellation: id <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Id {
    id: usize,
    index: std::num::NonZeroU32,
}

impl Id {
    pub fn new(id: usize, index: std::num::NonZeroU32) -> Self {
        Self { id, index }
    }

    pub(crate) fn next_index(&self) -> Self {
        Self {
            id: self.id,
            index: std::num::NonZeroU32::new(self.index.get() + 1).unwrap(),
        }
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{}.{}", self.index, self.id)
        } else {
            write!(f, "{}", self.index)
        }
    }
}

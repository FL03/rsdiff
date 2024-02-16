/*
    Appellation: id <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::AtomicId;
use daggy::NodeIndex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Id {
    id: AtomicId,
    index: NodeIndex,
}

impl Id {
    pub fn new(index: NodeIndex) -> Self {
        Self {
            id: AtomicId::new(),
            index,
        }
    }

    pub fn id(&self) -> usize {
        *self.id
    }

    pub fn index(&self) -> NodeIndex {
        self.index
    }

    pub(crate) fn next_index(&self) -> Self {
        Self {
            id: self.id,
            index: NodeIndex::new(self.index.index() + 1),
        }
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{}.{}", self.index.index(), self.id)
        } else {
            write!(f, "{}", self.index.index())
        }
    }
}

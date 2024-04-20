/*
    Appellation: id <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::AtomicId;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize,))]
pub struct IndexId<Idx = usize> {
    id: AtomicId,
    index: Idx,
}

impl<Idx> IndexId<Idx> {
    pub fn new(index: Idx) -> Self {
        Self {
            id: AtomicId::new(),
            index,
        }
    }

    pub fn id(&self) -> usize {
        *self.id
    }

    pub fn index(&self) -> &Idx {
        &self.index
    }
}

impl<Idx> core::fmt::Display for IndexId<Idx>
where
    Idx: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if f.alternate() {
            write!(f, "{}.{}", self.index(), self.id)
        } else {
            write!(f, "{}", self.index())
        }
    }
}

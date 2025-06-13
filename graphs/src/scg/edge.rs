/*
    Appellation: edge <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Id;
use crate::grad::GradientId;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize,))]
pub struct Edge<T> {
    id: Option<GradientId<T>>,
    data: T,
}

impl<T> Edge<T> {
    pub fn new(data: T, id: Option<GradientId<T>>) -> Self {
        Self { data, id }
    }

    pub fn constant(data: T) -> Self {
        Self { data, id: None }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn id(&self) -> Option<&GradientId<T>> {
        self.id.as_ref()
    }

    pub fn input(&self) -> Option<Id> {
        self.id().map(|id| **id)
    }
}

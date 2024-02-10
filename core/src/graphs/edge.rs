/*
    Appellation: edge <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::cmp::id::{GradientId, Id};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Value<T> {
    data: T,
    id: Option<GradientId<T>>,
}

impl<T> Value<T> {
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
        if let Some(id) = self.id() {
            Some(**id)
        } else {
            None
        }
    }
}

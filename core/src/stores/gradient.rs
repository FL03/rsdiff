/*
    Appellation: gradient <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Store;
use crate::graphs::FnGraph;
use daggy::NodeIndex;
use std::collections::BTreeMap;

pub struct GradientStore {
    store: BTreeMap<NodeIndex, Box<dyn std::any::Any>>,
}

impl GradientStore {
    pub fn new() -> Self {
        Self {
            store: BTreeMap::new(),
        }
    }
}

impl<T> Store<NodeIndex, T> for GradientStore
where
    T: Clone + 'static,
{
    fn get(&self, key: &NodeIndex) -> Option<&T> {
        self.store.get(key).map(|v| v.downcast_ref::<T>().unwrap())
    }

    fn get_mut(&mut self, key: &NodeIndex) -> Option<&mut T> {
        self.store
            .get_mut(key)
            .map(|v| v.downcast_mut::<T>().unwrap())
    }

    fn insert(&mut self, key: NodeIndex, value: T) {
        self.store.insert(key, Box::new(value));
    }

    fn remove(&mut self, key: &NodeIndex) -> Option<T> {
        self.store
            .remove(key)
            .map(|v| v.downcast_ref::<T>().unwrap().clone())
    }
}

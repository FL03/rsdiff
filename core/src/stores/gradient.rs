/*
    Appellation: gradient <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Store;
use petgraph::prelude::NodeIndex;
use std::any::Any;
use std::collections::BTreeMap;

pub struct GradientStore<K = NodeIndex> {
    store: BTreeMap<K, Box<dyn Any>>,
}

impl<K> GradientStore<K>
where
    K: Ord,
{
    pub fn new() -> Self {
        Self {
            store: BTreeMap::new(),
        }
    }

    pub fn or_insert(&mut self, key: K, value: Box<dyn Any>) -> &mut dyn Any {
        self.store.entry(key).or_insert(value)
    }
}

impl<K, T> Store<K, T> for GradientStore<K>
where
    K: Ord,
    T: Clone + 'static,
{
    fn get(&self, key: &K) -> Option<&T> {
        self.store.get(key).map(|v| v.downcast_ref::<T>().unwrap())
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut T> {
        self.store
            .get_mut(key)
            .map(|v| v.downcast_mut::<T>().unwrap())
    }

    fn insert(&mut self, key: K, value: T) -> Option<T> {
        self.store
            .insert(key, Box::new(value))
            .map(|v| v.downcast_ref::<T>().unwrap().clone())
    }

    fn remove(&mut self, key: &K) -> Option<T> {
        self.store
            .remove(key)
            .map(|v| v.downcast_ref::<T>().unwrap().clone())
    }
}

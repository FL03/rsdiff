/*
    Appellation: gradient <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::NodeIndex;
use acme::prelude::Store;
use std::any::Any;
use std::collections::btree_map::{BTreeMap, Entry};
use std::ops::{Index, IndexMut};

pub struct GradientStore<K = NodeIndex, V = Box<dyn Any>> {
    store: BTreeMap<K, V>,
}

impl<K, V> GradientStore<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        Self {
            store: BTreeMap::new(),
        }
    }

    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        self.store.entry(key)
    }

    pub fn or_insert(&mut self, key: K, value: V) -> &mut V {
        self.store.entry(key).or_insert(value)
    }
}

impl<K, T> Store<K, T> for GradientStore<K, T>
where
    K: Ord,
{
    fn get(&self, key: &K) -> Option<&T> {
        self.store.get(key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut T> {
        self.store.get_mut(key)
    }

    fn insert(&mut self, key: K, value: T) -> Option<T> {
        self.store.insert(key, value)
    }

    fn remove(&mut self, key: &K) -> Option<T> {
        self.store.remove(key)
    }
}

impl<K, T> Index<K> for GradientStore<K, T>
where
    K: Ord,
{
    type Output = T;

    fn index(&self, key: K) -> &Self::Output {
        self.store.get(&key).expect("Key not found")
    }
}

impl<K, T> IndexMut<K> for GradientStore<K, T>
where
    K: Ord,
{
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        self.store.get_mut(&key).expect("Key not found")
    }
}

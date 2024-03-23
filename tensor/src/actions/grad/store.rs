/*
    Appellation: store <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::TensorId;
use crate::TensorBase;
use acme::prelude::Store;
use std::collections::btree_map::{BTreeMap, Entry};
use std::ops::{Index, IndexMut};

pub struct GradStore<T> {
    pub(crate) store: BTreeMap<TensorId, TensorBase<T>>,
}

impl<T> GradStore<T> {
    pub fn new() -> Self {
        Self {
            store: BTreeMap::new(),
        }
    }
    /// Clears the store, removing all values.
    pub fn clear(&mut self) {
        self.store.clear()
    }
    /// Returns a reference to the value corresponding to the key.
    pub fn entry(&mut self, key: TensorId) -> Entry<'_, TensorId, TensorBase<T>> {
        self.store.entry(key)
    }
    /// Returns a reference to the value corresponding to the key.
    pub fn get_tensor(&self, item: &TensorBase<T>) -> Option<&TensorBase<T>> {
        self.store.get(&item.id())
    }
    /// Inserts a tensor into the store.
    pub fn insert_tensor(&mut self, tensor: TensorBase<T>) -> Option<TensorBase<T>> {
        self.insert(tensor.id, tensor)
    }
    /// Returns true if the store contains no elements.
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
    /// Returns the number of elements in the store.
    pub fn len(&self) -> usize {
        self.store.len()
    }
    /// If the store does not have a tensor with the given id, insert it.
    /// Returns a mutable reference to the tensor.
    pub fn or_insert(&mut self, tensor: TensorBase<T>) -> &mut TensorBase<T> {
        self.entry(tensor.id()).or_insert(tensor)
    }

    pub fn or_insert_default(&mut self, tensor: &TensorBase<T>) -> &mut TensorBase<T>
    where
        T: Clone + Default,
    {
        self.entry(tensor.id()).or_insert(tensor.default_like())
    }

    pub fn or_insert_zeros(&mut self, tensor: &TensorBase<T>) -> &mut TensorBase<T>
    where
        T: Clone + num::Zero,
    {
        self.entry(tensor.id()).or_insert(tensor.zeros_like())
    }
}

impl<T> Store<TensorId, TensorBase<T>> for GradStore<T> {
    fn get(&self, key: &TensorId) -> Option<&TensorBase<T>> {
        self.store.get(key)
    }

    fn get_mut(&mut self, key: &TensorId) -> Option<&mut TensorBase<T>> {
        self.store.get_mut(key)
    }

    fn insert(&mut self, key: TensorId, value: TensorBase<T>) -> Option<TensorBase<T>> {
        self.store.insert(key, value)
    }

    fn remove(&mut self, key: &TensorId) -> Option<TensorBase<T>> {
        self.store.remove(key)
    }
}

impl<T> Index<&TensorId> for GradStore<T> {
    type Output = TensorBase<T>;

    fn index(&self, index: &TensorId) -> &Self::Output {
        &self.store[index]
    }
}

impl<T> IndexMut<&TensorId> for GradStore<T> {
    fn index_mut(&mut self, index: &TensorId) -> &mut Self::Output {
        self.get_mut(index).expect("Tensor not found")
    }
}

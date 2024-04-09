/*
    Appellation: store <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::TensorId;
use crate::TensorBase;
use acme::prelude::Store;
use core::borrow::{Borrow, BorrowMut};
use core::ops::{Deref, DerefMut, Index, IndexMut};
use std::collections::btree_map::{BTreeMap, Entry, Keys, Values};

#[derive(Clone, Debug)]
pub struct TensorGrad<T> {
    pub(crate) store: BTreeMap<TensorId, TensorBase<T>>,
}

impl<T> TensorGrad<T> {
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
        self.insert(tensor.id(), tensor)
    }
    /// Returns true if the store contains no elements.
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
    /// Returns an iterator over the store's keys
    pub fn keys(&self) -> Keys<'_, TensorId, TensorBase<T>> {
        self.store.keys()
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
    /// If the store does not have a tensor with the given id, insert a tensor with the same shape
    /// and dtype as the given tensor, where all elements are default.
    pub fn or_insert_default(&mut self, tensor: &TensorBase<T>) -> &mut TensorBase<T>
    where
        T: Clone + Default,
    {
        self.entry(tensor.id()).or_insert(tensor.default_like())
    }
    /// If the store does not have a tensor with the given id, insert a tensor with the same shape
    /// and dtype as the given tensor, where all elements are zeros.
    pub fn or_insert_zeros(&mut self, tensor: &TensorBase<T>) -> &mut TensorBase<T>
    where
        T: Clone + num::Zero,
    {
        self.entry(tensor.id()).or_insert(tensor.zeros_like())
    }
    /// Remove an element from the store.
    pub fn remove(&mut self, key: &TensorId) -> Option<TensorBase<T>> {
        self.store.remove(key)
    }
    /// Remove a tensor from the store.
    pub fn remove_tensor(&mut self, tensor: &TensorBase<T>) -> Option<TensorBase<T>> {
        self.remove(&tensor.id())
    }

    pub fn values(&self) -> Values<'_, TensorId, TensorBase<T>> {
        self.store.values()
    }
}

impl<T> AsRef<BTreeMap<TensorId, TensorBase<T>>> for TensorGrad<T> {
    fn as_ref(&self) -> &BTreeMap<TensorId, TensorBase<T>> {
        &self.store
    }
}

impl<T> AsMut<BTreeMap<TensorId, TensorBase<T>>> for TensorGrad<T> {
    fn as_mut(&mut self) -> &mut BTreeMap<TensorId, TensorBase<T>> {
        &mut self.store
    }
}

impl<T> Borrow<BTreeMap<TensorId, TensorBase<T>>> for TensorGrad<T> {
    fn borrow(&self) -> &BTreeMap<TensorId, TensorBase<T>> {
        &self.store
    }
}

impl<T> BorrowMut<BTreeMap<TensorId, TensorBase<T>>> for TensorGrad<T> {
    fn borrow_mut(&mut self) -> &mut BTreeMap<TensorId, TensorBase<T>> {
        &mut self.store
    }
}

impl<T> Deref for TensorGrad<T> {
    type Target = BTreeMap<TensorId, TensorBase<T>>;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

impl<T> DerefMut for TensorGrad<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.store
    }
}

impl<T> Extend<(TensorId, TensorBase<T>)> for TensorGrad<T> {
    fn extend<I: IntoIterator<Item = (TensorId, TensorBase<T>)>>(&mut self, iter: I) {
        self.store.extend(iter)
    }
}

impl<T> FromIterator<(TensorId, TensorBase<T>)> for TensorGrad<T> {
    fn from_iter<I: IntoIterator<Item = (TensorId, TensorBase<T>)>>(iter: I) -> Self {
        Self {
            store: BTreeMap::from_iter(iter),
        }
    }
}

impl<T> Index<&TensorId> for TensorGrad<T> {
    type Output = TensorBase<T>;

    fn index(&self, index: &TensorId) -> &Self::Output {
        &self.store[index]
    }
}

impl<T> IndexMut<&TensorId> for TensorGrad<T> {
    fn index_mut(&mut self, index: &TensorId) -> &mut Self::Output {
        self.get_mut(index).expect("Tensor not found")
    }
}

impl<T> IntoIterator for TensorGrad<T> {
    type Item = (TensorId, TensorBase<T>);
    type IntoIter = std::collections::btree_map::IntoIter<TensorId, TensorBase<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.store.into_iter()
    }
}

impl<T> Store<TensorId, TensorBase<T>> for TensorGrad<T> {
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
        self.remove(key)
    }
}

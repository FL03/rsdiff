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

use ndarray::{DataOwned, RawData};

pub struct TensorGrad<S>
where
    S: RawData,
{
    pub(crate) store: BTreeMap<TensorId, TensorBase<S>>,
}

impl<S> Default for TensorGrad<S>
where
    S: RawData,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<A, S> TensorGrad<S>
where
    S: RawData<Elem = A>,
{
    pub fn new() -> Self {
        Self {
            store: BTreeMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.store.clear()
    }

    pub fn entry(&mut self, key: TensorId) -> Entry<'_, TensorId, TensorBase<S>> {
        self.store.entry(key)
    }

    pub fn get_mut(&mut self, key: &TensorId) -> Option<&mut TensorBase<S>> {
        self.store.get_mut(key)
    }

    pub fn get_tensor(&self, item: &TensorBase<S>) -> Option<&TensorBase<S>> {
        self.store.get(&item.id())
    }

    pub fn insert(&mut self, key: TensorId, tensor: TensorBase<S>) -> Option<TensorBase<S>> {
        self.store.insert(key, tensor)
    }

    pub fn insert_tensor(&mut self, tensor: TensorBase<S>) -> Option<TensorBase<S>> {
        self.insert(tensor.id(), tensor)
    }

    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    pub fn keys(&self) -> Keys<'_, TensorId, TensorBase<S>> {
        self.store.keys()
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn or_insert(&mut self, tensor: TensorBase<S>) -> &mut TensorBase<S> {
        self.entry(tensor.id()).or_insert(tensor)
    }

    pub fn or_insert_with<F>(&mut self, key: TensorId, default: F) -> &mut TensorBase<S>
    where
        F: FnOnce() -> TensorBase<S>,
    {
        self.entry(key).or_insert_with(default)
    }

    pub fn or_insert_default(&mut self, tensor: &TensorBase<S>) -> &mut TensorBase<S>
    where
        A: Clone + Default,
        S: DataOwned,
    {
        self.entry(tensor.id()).or_insert(tensor.default_like())
    }

    pub fn or_insert_zeros(&mut self, tensor: &TensorBase<S>) -> &mut TensorBase<S>
    where
        A: Clone + num::Zero,
        S: DataOwned,
    {
        self.entry(tensor.id()).or_insert(tensor.zeros_like())
    }

    pub fn remove(&mut self, key: &TensorId) -> Option<TensorBase<S>> {
        self.store.remove(key)
    }

    pub fn remove_tensor(&mut self, tensor: &TensorBase<S>) -> Option<TensorBase<S>> {
        self.remove(&tensor.id())
    }

    pub fn values(&self) -> Values<'_, TensorId, TensorBase<S>> {
        self.store.values()
    }

    pub fn values_mut(
        &mut self,
    ) -> std::collections::btree_map::ValuesMut<TensorId, TensorBase<S>> {
        self.store.values_mut()
    }
}

impl<S> Deref for TensorGrad<S>
where
    S: RawData,
{
    type Target = BTreeMap<TensorId, TensorBase<S>>;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

impl<S> DerefMut for TensorGrad<S>
where
    S: RawData,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.store
    }
}

impl<S> Index<TensorId> for TensorGrad<S>
where
    S: RawData,
{
    type Output = TensorBase<S>;

    fn index(&self, index: TensorId) -> &Self::Output {
        &self.store[&index]
    }
}

impl<S> IndexMut<TensorId> for TensorGrad<S>
where
    S: RawData,
{
    fn index_mut(&mut self, index: TensorId) -> &mut Self::Output {
        self.get_mut(&index).unwrap()
    }
}

impl<S> Borrow<BTreeMap<TensorId, TensorBase<S>>> for TensorGrad<S>
where
    S: RawData,
{
    fn borrow(&self) -> &BTreeMap<TensorId, TensorBase<S>> {
        &self.store
    }
}

impl<S> BorrowMut<BTreeMap<TensorId, TensorBase<S>>> for TensorGrad<S>
where
    S: RawData,
{
    fn borrow_mut(&mut self) -> &mut BTreeMap<TensorId, TensorBase<S>> {
        &mut self.store
    }
}

impl<S> IntoIterator for TensorGrad<S>
where
    S: RawData,
{
    type Item = (TensorId, TensorBase<S>);
    type IntoIter = std::collections::btree_map::IntoIter<TensorId, TensorBase<S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.store.into_iter()
    }
}

impl<S> Extend<(TensorId, TensorBase<S>)> for TensorGrad<S>
where
    S: RawData,
{
    fn extend<I: IntoIterator<Item = (TensorId, TensorBase<S>)>>(&mut self, iter: I) {
        self.store.extend(iter)
    }
}

impl<S> FromIterator<(TensorId, TensorBase<S>)> for TensorGrad<S>
where
    S: RawData,
{
    fn from_iter<I: IntoIterator<Item = (TensorId, TensorBase<S>)>>(iter: I) -> Self {
        Self {
            store: BTreeMap::from_iter(iter),
        }
    }
}

impl<S> Store<TensorId, TensorBase<S>> for TensorGrad<S>
where
    S: RawData,
{
    fn get(&self, key: &TensorId) -> Option<&TensorBase<S>> {
        self.store.get(key)
    }

    fn get_mut(&mut self, key: &TensorId) -> Option<&mut TensorBase<S>> {
        self.store.get_mut(key)
    }

    fn insert(&mut self, key: TensorId, value: TensorBase<S>) -> Option<TensorBase<S>> {
        self.store.insert(key, value)
    }

    fn remove(&mut self, key: &TensorId) -> Option<TensorBase<S>> {
        self.store.remove(key)
    }
}

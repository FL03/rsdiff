/*
    Appellation: stores <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::id::Identify;
#[cfg(not(feature = "std"))]
use alloc::collections::{btree_map, BTreeMap};
use core::borrow::Borrow;
#[cfg(feature = "std")]
use std::collections::{btree_map, hash_map, BTreeMap, HashMap};

pub trait Entry<'a> {
    type Key;
    type Value;

    fn key(&self) -> &Self::Key;

    fn or_insert(self, default: Self::Value) -> &'a mut Self::Value;
}

pub trait Get<Q> {
    type Key: Borrow<Q>;
    type Value;

    fn get(&self, key: &Q) -> Option<&Self::Value>;
}

pub trait GetMut<Q>: Get<Q> {
    fn get_mut(&mut self, key: &Q) -> Option<&mut Self::Value>;
}

impl<Q, K, V> Get<Q> for BTreeMap<K, V>
where
    K: Borrow<Q> + Ord,
    Q: Ord,
{
    type Key = K;
    type Value = V;

    fn get(&self, key: &Q) -> Option<&Self::Value> {
        BTreeMap::get(self, key)
    }
}

pub trait Store<K, V> {
    fn get(&self, key: &K) -> Option<&V>;

    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    fn insert(&mut self, key: K, value: V) -> Option<V>;

    fn remove(&mut self, key: &K) -> Option<V>;
}

pub trait StoreExt<T>
where
    T: Identify,
    <T as Identify>::Id: Copy,
{
    type Store: Store<T::Id, T>;

    fn store(&self) -> &Self::Store;

    fn store_mut(&mut self) -> &mut Self::Store;

    fn get(&self, item: T) -> Option<&T> {
        Store::get(self.store(), item.id())
    }

    fn get_mut(&mut self, item: T) -> Option<&mut T> {
        Store::get_mut(self.store_mut(), item.id())
    }

    fn insert(&mut self, item: T) -> Option<T> {
        Store::insert(self.store_mut(), *item.id(), item)
    }

    fn remove(&mut self, item: T) -> Option<T> {
        Store::remove(self.store_mut(), item.id())
    }
}

impl<S, T> StoreExt<T> for S
where
    S: Store<T::Id, T>,
    T: Identify,
    <T as Identify>::Id: Copy,
{
    type Store = S;

    fn store(&self) -> &Self::Store {
        self
    }

    fn store_mut(&mut self) -> &mut Self::Store {
        self
    }
}

macro_rules! entry {
    ($($prefix:ident)::*.$call:ident($($arg:tt),*)) => {
        $($prefix)::*::Entry::$call($($arg),*)
    };

}

macro_rules! impl_entry {
    ($($prefix:ident)::* where $($preds:tt)* ) => {
        impl<'a, K, V> Entry<'a> for $($prefix)::*::Entry<'a, K, V> where $($preds)* {
            type Key = K;
            type Value = V;

            fn key(&self) -> &Self::Key {
                entry!($($prefix)::*.key(self))
            }

            fn or_insert(self, default: Self::Value) -> &'a mut Self::Value {
                entry!($($prefix)::*.or_insert(self, default))
            }
        }
    };

}

macro_rules! impl_store {
    ($t:ty where $($rest:tt)*) => {

        impl<K, V> Store<K, V> for $t where $($rest)* {
            fn get(&self, key: &K) -> Option<&V> {
                <$t>::get(self, &key)
            }

            fn get_mut(&mut self, key: &K) -> Option<&mut V> {
                <$t>::get_mut(self, &key)
            }

            fn insert(&mut self, key: K, value: V) -> Option<V> {
                <$t>::insert(self, key, value)
            }

            fn remove(&mut self, key: &K) -> Option<V> {
                <$t>::remove(self, &key)
            }
        }

    };
}

impl_entry!(btree_map where K: Ord);
#[cfg(feature = "std")]
impl_entry!(hash_map where K: Eq + core::hash::Hash);
impl_store!(BTreeMap<K, V> where K: Ord);
#[cfg(feature = "std")]
impl_store!(HashMap<K, V> where K: Eq + core::hash::Hash);

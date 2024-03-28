/*
    Appellation: stores <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap};

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

pub trait Cache<K, V> {
    fn get_or_insert_with<F>(&mut self, key: K, f: F) -> &mut V
    where
        F: FnOnce() -> V;
}

pub trait OrInsert<K, V> {
    fn or_insert(&mut self, key: K, value: V) -> &mut V;
}

macro_rules! impl_store {
    ($t:ty, where $($preds:tt)* ) => {

        impl<K, V> Store<K, V> for $t where $($preds)* {
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

impl_store!(BTreeMap<K, V>, where K: Ord);
impl_store!(HashMap<K, V>, where K: Eq + std::hash::Hash);

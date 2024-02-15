/*
    Appellation: stores <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::gradient::*;

pub(crate) mod gradient;

use std::collections::{BTreeMap, HashMap};

pub trait Store<K, V> {
    fn get(&self, key: &K) -> Option<&V>;

    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    fn insert(&mut self, key: K, value: V) -> Option<V>;

    fn remove(&mut self, key: &K) -> Option<V>;
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